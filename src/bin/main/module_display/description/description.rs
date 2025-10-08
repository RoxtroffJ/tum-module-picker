//! The description of the description ;)

use std::sync::Arc;

use iced::{
    Alignment::*,
    Element,
    advanced::graphics::futures::MaybeSend,
    widget::{row, text, text_editor},
};
use tum_module_picker::{lines::Lines, module::Module, sectioned_lines::get_sections};

use crate::{module_display::{Edit, Resetable}, *};

/// The content of the description.
#[derive(Debug)]
pub struct Content {
    content: Option<Editable>,
}

editable_maker! {
    pub,
    Editable,

    prerequisites,
    intended_learning_outcomes, content, teaching_and_learning_methods,
    media, reading_list, responsible_bis;
    ;
    [intended_learning_outcomes_editor {intended_learning_outcomes}];
    [content_editor {content}];
    [teaching_and_learning_methods_editor {teaching_and_learning_methods}];
    [reading_list_editor {reading_list}];

    editor text_editor::Content = text_editor::Content::new()
}

/// The actions performed by the [perform](Content::perform) function.
#[derive(Debug, Clone)]
pub enum Action {
    Prerequisites(String),

    IntendedLearningOutcomes(text_editor::Action),
    Content(text_editor::Action),
    TeachingAndLearningMethods(text_editor::Action),

    Media(String),
    ReadingList(text_editor::Action),
    ResponsibleBis(String),

    Editor(text_editor::Action),
}

impl Content {
    /// Creates a new description content.
    pub fn new() -> Self {
        Self { content: None }
    }

    /// Returns a reference to the inner [Editable].
    pub fn get_editable(&self) -> &Option<Editable> {
        &self.content
    }

    /// Returns a mutable reference to the inner [Editable].
    pub fn get_mut_editable(&mut self) -> &mut Option<Editable> {
        &mut self.content
    }

    /// Makes all the fields editable or not.
    pub fn set_all_edits(&mut self, value: bool, module: &Module) {
        if !value && self.get_editable().is_none() {
            return;
        }

        self.get_mut_editable()
            .get_or_insert(Editable::new(module))
            .set_all(value)
    }

    /// Views the description field.
    pub fn view<'a>(&'a self, module: &'a Module) -> Element<'a, Action> {
        macro_rules! texter_row {
            ($label:expr, $field:ident, $msg:expr) => {
                row![
                    bald_text($label),
                    texter!(module, &self.content, text, $field, $msg)
                ]
                .align_y(Center)
            };
        }

        macro_rules! editor_row {
            ($label:expr, $field:ident, $editor_field:ident, $msg:expr) => {
                content_column![
                    bald_text($label),
                    editor_texter!(module, &self.content, text, $field, $editor_field, $msg)
                ]
            };
        }

        content_column![
            texter_row!(
                "Prerequisites (recommended): ",
                prerequisites,
                Action::Prerequisites
            ),
            editor_row!(
                "Intended Learning Outcomes: ",
                intended_learning_outcomes,
                intended_learning_outcomes_editor,
                Action::IntendedLearningOutcomes
            ),
            editor_row!("Content: ", content, content_editor, Action::Content),
            editor_row!(
                "Teaching and Learning Methods: ",
                teaching_and_learning_methods,
                teaching_and_learning_methods_editor,
                Action::TeachingAndLearningMethods
            ),
            texter_row!("Media: ", media, Action::Media),
            editor_row!("Reading List: ", reading_list, reading_list_editor, Action::ReadingList),
            texter_row!("Responsible for module: ", responsible_bis, Action::ResponsibleBis)
        ]
        .into()
    }

    /// Performs the action.
    pub fn perform(&mut self, action: Action, module: &mut Module) -> Task<Action> {
        macro_rules! set {
            ($str:expr, $field: ident) => {
                set_str_field!(module, self.get_mut_editable(), $str, $field)
            };
        }

        macro_rules! set_editor {
            ($action:expr, $field:ident, $editor_field:ident) => {
                set_editor_field!(
                    module,
                    self.get_mut_editable(),
                    $action,
                    $field,
                    $editor_field
                )
            };
        }

        match action {
            Action::Prerequisites(str) => set!(str, prerequisites),
            Action::IntendedLearningOutcomes(action) => set_editor!(
                action,
                intended_learning_outcomes,
                intended_learning_outcomes_editor
            ),
            Action::Content(action) => set_editor!(action, content, content_editor),
            Action::TeachingAndLearningMethods(action) => set_editor!(
                action,
                teaching_and_learning_methods,
                teaching_and_learning_methods_editor
            ),
            Action::Media(str) => set!(str, media),
            Action::ReadingList(str) => set_editor!(str, reading_list, reading_list_editor),
            Action::ResponsibleBis(str) => set!(str, responsible_bis),
            Action::Editor(action) => return editor_perform(self.get_mut_editable(), action),
        }

        Task::none()
    }

    /// Resets the fields to match those of the given [Module].
    pub fn reset(&mut self, module: &Module) {
        self.content.as_mut().map(|x| x.reset(module));
    }
}

enum Section {
    Prerequisites,
    IntendedLearningOutcomes,
    Content,
    TeachingAndLearningMethods,
    Media,
    ReadingList,
}

fn editor_perform(editable: &mut Option<Editable>, action: text_editor::Action) -> Task<Action> {
    if let Some(editable) = editable {
        editable.editor.perform(action);

        let mut tasks = vec![];

        let mut sections = get_sections(editable.editor.lines(), &|str| {
            use Section::*;
            match str {
                "Prerequisites (recommended)" => Some(Prerequisites),
                "Intended Learning Outcomes" => Some(IntendedLearningOutcomes),
                "Content" => Some(Content),
                "Teaching and Learning Methods" => Some(TeachingAndLearningMethods),
                "Media" => Some(Media),
                "Reading List" => Some(ReadingList),
                _ => None,
            }
        });

        while let Some(sec) = sections.next_section() {
            match sec {
                Section::Prerequisites => push_string(&mut tasks, &mut sections, Action::Prerequisites),
                Section::IntendedLearningOutcomes => push_editor(&mut tasks, &mut sections, Action::IntendedLearningOutcomes),
                Section::Content => push_editor(&mut tasks, &mut sections, Action::Content),
                Section::TeachingAndLearningMethods => push_editor(&mut tasks, &mut sections, Action::TeachingAndLearningMethods),
                Section::Media => push_string(&mut tasks, &mut sections, Action::Media),
                Section::ReadingList => push_editor(&mut tasks, &mut sections, Action::ReadingList),
            }
        }

        Task::batch(tasks)
    } else {
        Task::none()
    }
}

fn push_editor(
    tasks: &mut Vec<Task<Action>>,
    sections: &mut impl Lines,
    action: impl Fn(text_editor::Action) -> Action + MaybeSend + 'static,
) {
    tasks.push(
        Task::batch(vec![
            Task::done(text_editor::Action::SelectAll),
            Task::done(text_editor::Action::Edit(text_editor::Edit::Delete)),
            Task::done(text_editor::Action::Edit(text_editor::Edit::Paste(
                Arc::new(sections.text()),
            ))),
        ])
        .map(action),
    )
}

fn push_string(
    tasks: &mut Vec<Task<Action>>,
    sections: &mut impl Lines,
    action: impl Fn(String) -> Action + MaybeSend + 'static,
) {
    tasks.push(
        Task::done(action(sections.next().as_deref().unwrap_or_default().to_string()))
    );
}