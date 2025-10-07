//! Study subsection of description

use std::{
    fmt::Display, ops::Deref, sync::{Arc, LazyLock}
};

use crate::{
    bald_text, content_column, editable_maker, editor_texter,
    module_display::Edit,
    non_str_texter, set_editor_field, set_non_str_field,
};
use iced::{
    Alignment::Center,
    Element, Task,
    widget::{row, text, text_editor},
};
use regex::Regex;
use tum_module_picker::{lines::Lines, module::Module, sectioned_lines::get_sections};

/// Content of the study section
#[derive(Debug)]
pub struct Content {
    content: Option<Editable>,
}

editable_maker! {
    pub,
    Editable,
    descr_of_achievement_assessment_methods,
    exam_retake_next_semester,
    exam_retake_end_semester;
    exam_retake_next_semester_str {exam_retake_next_semester bool_to_string} exam_retake_next_semester_err ParseError,
    exam_retake_end_semester_str {exam_retake_end_semester bool_to_string} exam_retake_end_semester_err ParseError;
    [descr_of_achievement_assessment_methods_editor {descr_of_achievement_assessment_methods}];
    editor text_editor::Content = text_editor::Content::new();
}

#[derive(Debug, Clone)]
pub enum Action {
    Desc(text_editor::Action),
    ExamNextSem(String),
    ExamEndSem(String),

    Editor(text_editor::Action),
}

impl Content {
    /// Creates a new [Content].
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

    /// Views the content.
    pub fn view<'a>(&'a self, module: &'a Module) -> Element<'a, Action> {
        macro_rules! bool_texter {
            ($label:expr, $field:ident, $str_field:ident, $err_field:ident, $msg:expr) => {
                row![
                    bald_text($label),
                    non_str_texter!(
                        module,
                        &self.content,
                        text,
                        $field,
                        $str_field,
                        $err_field,
                        $msg
                    ),
                ]
                .align_y(Center)
            };
        }

        content_column![
            content_column![
                bald_text("Description of Achievement and Assessment Methods:"),
                editor_texter!(
                    module,
                    &self.content,
                    text,
                    descr_of_achievement_assessment_methods,
                    descr_of_achievement_assessment_methods_editor,
                    Action::Desc
                )
            ],
            bool_texter!(
                "Exam retake next semester: ",
                exam_retake_next_semester,
                exam_retake_next_semester_str,
                exam_retake_next_semester_err,
                Action::ExamNextSem
            ),
            bool_texter!(
                "Exam retake at the end of the semester: ",
                exam_retake_end_semester,
                exam_retake_end_semester_str,
                exam_retake_end_semester_err,
                Action::ExamEndSem
            ),
        ]
        .into()
    }

    /// Performs the action.
    pub fn perform(&mut self, action: Action, module: &mut Module) -> Task<Action> {
        match action {
            Action::Desc(action) => set_editor_field!(
                module,
                &mut self.content,
                action,
                descr_of_achievement_assessment_methods,
                descr_of_achievement_assessment_methods_editor
            ),
            Action::ExamNextSem(str) => set_non_str_field!(
                module,
                &mut self.content,
                parse_bool(&str),
                str,
                exam_retake_next_semester,
                exam_retake_next_semester_str,
                exam_retake_next_semester_err
            ),
            Action::ExamEndSem(str) => set_non_str_field!(
                module,
                &mut self.content,
                parse_bool(&str),
                str,
                exam_retake_end_semester,
                exam_retake_end_semester_str,
                exam_retake_end_semester_err
            ),
            Action::Editor(action) => return perform_editor(&mut self.content, action),
        }
        Task::none()
    }
}

enum Section {
    Descr,
    Next,
    End,
}

fn perform_editor(editable: &mut Option<Editable>, action: text_editor::Action) -> Task<Action> {
    if let Some(editable) = editable {
        editable.editor.perform(action);

        let mut tasks = vec![];

        let mut sections = get_sections(editable.editor.lines(), &|str| match str {
            "Description of Achievement and Assessment Methods" => Some(Section::Descr),
            "Exam retake next semester" => Some(Section::Next),
            "Exam retake at the end of semester" => Some(Section::End),
            _ => None,
        });

        while let Some(sec) = sections.next_section() {
            match sec {
                Section::Descr => tasks.push(
                    Task::batch(vec![
                        Task::done(text_editor::Action::SelectAll),
                        Task::done(text_editor::Action::Edit(text_editor::Edit::Delete)),
                        Task::done(text_editor::Action::Edit(text_editor::Edit::Paste(
                            Arc::new(sections.text()),
                        ))),
                    ])
                    .map(Action::Desc),
                ),
                Section::Next => push_bool(&mut sections, &mut tasks, Action::ExamNextSem),
                Section::End => push_bool(&mut sections, &mut tasks, Action::ExamEndSem),
            }
        }

        Task::batch(tasks)
    } else {
        Task::none()
    }
}

fn push_bool(sections: &mut impl Iterator<Item: Deref<Target = str>>, tasks: &mut Vec<Task<Action>>, action: impl Fn(String) -> Action) {
    tasks.push(Task::done(action(sections.next().as_deref().unwrap_or_default().to_string())));
}

#[derive(Debug)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't read a boolean. Write 'yes' or 'no'.")
    }
}

static TRUE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b\<(?i)(y|yes|true)\b\>").unwrap());
static FALSE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b\<(?i)(n|no|false)\b\>").unwrap());
fn parse_bool(str: &str) -> Result<bool, ParseError> {
    if TRUE_REGEX.is_match(str) {
        return Ok(true);
    }
    if FALSE_REGEX.is_match(str) {
        return Ok(false);
    }
    return Err(ParseError);
}

fn bool_to_string(bool: &bool) -> String {
    if *bool { "Yes" } else { "No" }.to_string()
}
