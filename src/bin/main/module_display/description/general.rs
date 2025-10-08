//! The general section of the description
use iced::{
    Alignment::Center,
    Element, Task,
    widget::{row, text, text_editor},
};
use tum_module_picker::{module::Module, sectioned_lines::get_sections};

use crate::{
    bald_text, content_column, editable_maker, module_display::{Edit, Resetable}, set_str_field, texter,
};

/// Content of the general section.
#[derive(Debug)]
pub struct Content {
    content: Option<Editable>,
}

editable_maker! {
    pub,
    Editable Default,

    module_level, abbreviation, subtitle, duration, occurence, language, related_programs;
    editor_content text_editor::Content = text_editor::Content::new()
}

/// The actions performes by the [perform][Content::perform] function.
#[derive(Debug, Clone)]
pub enum Action {
    ModuleLevel(String),
    Abbreviation(String),
    Subtitle(String),
    Duration(String),
    Occurence(String),
    Language(String),
    RelatedPrograms(String),

    Editor(text_editor::Action),
}

impl Content {
    pub fn new() -> Self {
        Self { content: None }
    }

    pub fn view<'a>(&self, module: &'a Module) -> Element<'a, Action> {
        macro_rules! simple_texter {
            ($label:expr, $field:ident, $msg:expr) => {
                row![
                    bald_text($label),
                    texter!(module, &self.content, text, $field, $msg)
                ]
                .align_y(Center)
            };
        }

        content_column![
            simple_texter!("Module level: ", module_level, Action::ModuleLevel),
            simple_texter!("Abbreviation: ", abbreviation, Action::Abbreviation),
            simple_texter!("Subtitle: ", subtitle, Action::Subtitle),
            simple_texter!("Duration: ", duration, Action::Duration),
            simple_texter!("Occurence: ", occurence, Action::Occurence),
            simple_texter!("Language: ", language, Action::Language),
            simple_texter!(
                "Related programs: ",
                related_programs,
                Action::RelatedPrograms
            )
        ]
        .into()
    }

    /// Performs an action.
    pub fn perform(&mut self, action: Action, module: &mut Module) -> Task<Action> {
        macro_rules! set {
            ($str:expr, $field:ident) => {
                set_str_field!(module, &self.content, $str, $field)
            };
        }
        match action {
            Action::ModuleLevel(str) => set!(str, module_level),
            Action::Abbreviation(str) => set!(str, abbreviation),
            Action::Subtitle(str) => set!(str, subtitle),
            Action::Duration(str) => set!(str, duration),
            Action::Occurence(str) => set!(str, occurence),
            Action::Language(str) => set!(str, language),
            Action::RelatedPrograms(str) => set!(str, related_programs),
            Action::Editor(action) => return editor_perform(self.get_mut_editable(), action),
        };

        Task::none()
    }

    /// Returns the editable struct.
    pub fn get_editable(&self) -> &Option<Editable> {
        &self.content
    }

    /// Returns a mutable reference to the editable struct.
    pub fn get_mut_editable(&mut self) -> &mut Option<Editable> {
        &mut self.content
    }

    /// Makes every field editable or not.
    pub fn set_all_edits(&mut self, value: bool) {
        if self.get_editable().is_none() && !value {
            return;
        }

        self.get_mut_editable()
            .get_or_insert(Editable::new())
            .set_all(value)
    }

    /// Resets the fields to match those of the given [Module].
    pub fn reset(&mut self, module: &Module) {
        self.content.as_mut().map(|x| x.reset(module));
    }
}

enum Section {
    ModuleLevel,
    Abbreviation,
    Subtitle,
    Duration,
    Occurence,
    Language,
    RelatedPrograms,
}

impl Section {
    fn to_action(&self, str: String) -> Action {
        match self {
            Section::ModuleLevel => Action::ModuleLevel(str),
            Section::Abbreviation => Action::Abbreviation(str),
            Section::Subtitle => Action::Subtitle(str),
            Section::Duration => Action::Duration(str),
            Section::Occurence => Action::Occurence(str),
            Section::Language => Action::Language(str),
            Section::RelatedPrograms => Action::RelatedPrograms(str),
        }
    }
}

fn editor_perform(editable: &mut Option<Editable>, action: text_editor::Action) -> Task<Action> {
    if let Some(editable) = editable {
        editable.editor_content.perform(action);

        use Section::*;

        let mut sections = get_sections(editable.editor_content.lines(), &|str| match str {
            "Module Level" => Some(ModuleLevel),
            "Abbrevation" => Some(Abbreviation),
            "Subtitle" => Some(Subtitle),
            "Duration" => Some(Duration),
            "Occurrence" => Some(Occurence),
            "Language" => Some(Language),
            "Related Programs" => Some(RelatedPrograms),
            _ => None,
        });

        let mut tasks = vec![];

        while let Some(section) = sections.next_section() {
            tasks.push(Task::done(section.to_action(
                sections.next().as_deref().unwrap_or_default().to_string(),
            )));
        }

        Task::batch(tasks)
    } else {
        Task::none()
    }
}
