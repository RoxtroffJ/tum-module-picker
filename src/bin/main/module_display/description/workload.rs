//! The workload section of the description.

use std::num::ParseIntError;

use iced::{widget::{row, text_editor}, Element, Task, Alignment::Center};
use time::Duration;
use tum_module_picker::{module::Module, sectioned_lines::get_sections};

use crate::{bald_text, content_column, editable_maker, module_display::{Edit, Resetable}, non_str_texter, set_non_str_field};

/// Content of the workload section.
#[derive(Debug)]
pub struct Content {
    content: Option<Editable>
}

editable_maker!{
    pub,
    Editable,
    total_hours, contact_hours, self_study_hours;

    total_hours_str {total_hours hours_to_string} total_hours_err ParseError,
    contact_hours_str {contact_hours hours_to_string} contact_hours_err ParseError,
    self_study_hours_str {self_study_hours hours_to_string} self_study_hours_err ParseError;

    editor text_editor::Content = text_editor::Content::new()
}

/// Actions performed by the [perform](Content::perform) function.
#[derive(Debug, Clone)]
pub enum Action {
    TotalHours(String),
    ContactHours(String),
    SelfStudyHours(String),

    Editor(text_editor::Action),
}

impl Content {
    pub fn new() -> Self {
        Self { content: None }
    }

    pub fn get_editable(&self) -> &Option<Editable> {
        &self.content
    }

    pub fn get_mut_editable(&mut self) -> &mut Option<Editable> {
        &mut self.content
    }

    pub fn set_all_edits(&mut self, value: bool, module: &Module) {
        if !value && self.get_editable().is_none() {
            return;
        }

        self.get_mut_editable().get_or_insert(Editable::new(module)).set_all(value);
    }

    pub fn view(&self, module: &Module) -> Element<'_, Action> {
        macro_rules! custom_texter {
            ($label:expr, $field:ident, $str_field:ident, $err_field:ident, $msg:ident) => {
                row![
                    bald_text($label),
                    non_str_texter!(module, &self.content, |duration| text(hours_to_string(duration)), $field, $str_field, $err_field, Action::$msg)
                ].align_y(Center)
            };
        }

        content_column![
               custom_texter!("Total hours: ", total_hours, total_hours_str, total_hours_err, TotalHours),
               custom_texter!("Contact hours: ", contact_hours, contact_hours_str, contact_hours_err, ContactHours),
               custom_texter!("Self study hours: ", self_study_hours, self_study_hours_str, self_study_hours_err, SelfStudyHours),
        ].into()
    }

    pub fn perform(&mut self, action: Action, module: &mut Module) -> Task<Action> {
        macro_rules! set {
            ($str:expr, $field: ident, $field_string:ident, $field_error:ident) => {
                set_non_str_field!(module, &mut self.content, parse_duration(&$str), $str, $field, $field_string, $field_error)
            };
        }
        
        match action {
            Action::TotalHours(str) => set!(str, total_hours, total_hours_str, total_hours_err),
            Action::ContactHours(str) => set!(str, contact_hours, contact_hours_str, contact_hours_err),
            Action::SelfStudyHours(str) => set!(str, self_study_hours, self_study_hours_str, self_study_hours_err),
            Action::Editor(action) => return perform_editor(self.get_mut_editable(), action),
        };

        return Task::none();
    }

    /// Resets the fields to match those of the given [Module].
    pub fn reset(&mut self, module: &Module) {
        self.content.as_mut().map(|x| x.reset(module));
    }
}

enum Sec {
    Total,
    Contact,
    SelfStudy,
}

impl Sec {
    fn to_action(&self, str: String) -> Action {
        match self {
            Sec::Total => Action::TotalHours(str),
            Sec::Contact => Action::ContactHours(str),
            Sec::SelfStudy => Action::SelfStudyHours(str),
        }
    }
}

fn perform_editor(editable: &mut Option<Editable>, action: text_editor::Action) -> Task<Action> {
    if let Some(editable) = editable {
        editable.editor.perform(action);

        let mut tasks = Vec::new();

        let mut sections = get_sections(editable.editor.lines(), &|str| {
            use Sec::*;
            match str {
                "Total Hours" => Some(Total),
                "Contact Hours" => Some(Contact),
                "Self-study Hours" => Some(SelfStudy),
                _ => None
            }
        });

        while let Some(sec) = sections.next_section() {
            tasks.push(Task::done(sec.to_action(sections.next().as_deref().unwrap_or_default().to_string())))
        };

        Task::batch(tasks)
    } else {
        Task::none()
    }
}

fn hours_to_string(duration: &Duration) -> String {
    duration.whole_hours().to_string()
}

type ParseError = ParseIntError;

fn parse_duration(string: &String) -> Result<Duration, ParseError> {
    Ok(Duration::hours(string.parse()?))
}