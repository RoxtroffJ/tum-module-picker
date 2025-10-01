use std::{
    borrow::{Borrow, BorrowMut},
    num::ParseIntError,
    ops::{Deref, DerefMut},
};

use super::*;
use crate::{MENU_OFFSET, PADDING};
use iced::{
    Alignment::{self, Center},
    Element,
    Length::*,
    widget::{
        TextInput, column, container, horizontal_rule, horizontal_space, row, text, text_input,
    },
};
use st_column::Column as STColumn;
use storage_tree::column as st_column;
use storage_tree::column::Content as STContent;
use tum_module_picker::{
    module::module::Module,
    storage_tree::{self, StorageTree},
};

/// State of a [ModuleDisplay].
///
/// It can be [Deref]ed in a [Module].
#[derive(Debug)]
pub struct Content {
    module: Module,
    overview_content: STContent<(), ()>,
    editable: Option<Editable>,
}

#[derive(Debug, Default)]
struct Editable {
    // === Overview ===
    name: bool,
    id: bool,
    ects: bool,
    version: bool,
    valid_from: bool,
    valid_until: bool,
    responsible: bool,
    organisation: bool,
    note: bool,

    // === Courses and exams ===
    courses: bool,
    exams: bool,

    // === Description ===
    // General data
    module_level: bool,
    abbreviation: bool,
    subtitle: bool,
    duration: bool,
    occurence: bool,
    language: bool,
    related_programs: bool,

    // Work load
    total_hours: bool,
    contact_hours: bool,
    self_study_hours: bool,

    // Study and examination performance
    descr_of_achievement_assessment_methods: bool,
    exam_retake_next_semester: bool,
    exam_retake_end_semester: bool,

    // Description
    prerequisites: bool,
    intended_learning_outcomes: bool,
    content: bool,
    teaching_and_learning_methods: bool,
    media: bool,
    reading_list: bool,

    // Responsible for module
    responsible_bis: bool,

    // Values for non string fields
    ects_string: String,
    ects_error: Option<ParseIntError>,
}

impl Editable {
    /// Sets all the edits to something
    pub fn set_all(self, value: bool) -> Self {
        Self {
            name: value,
            id: value,
            ects: value,
            version: value,
            valid_from: value,
            valid_until: value,
            responsible: value,
            organisation: value,
            note: value,

            courses: value,
            exams: value,

            module_level: value,
            abbreviation: value,
            subtitle: value,
            duration: value,
            occurence: value,
            language: value,
            related_programs: value,

            total_hours: value,
            contact_hours: value,
            self_study_hours: value,

            descr_of_achievement_assessment_methods: value,
            exam_retake_next_semester: value,
            exam_retake_end_semester: value,

            prerequisites: value,
            intended_learning_outcomes: value,
            content: value,
            teaching_and_learning_methods: value,
            media: value,
            reading_list: value,

            responsible_bis: value,

            ..self
        }
    }
}

/// Struct that implements [Into<Element>].
pub struct ModuleDisplay<'a> {
    content: &'a Content,
}

/// Actions that [Content] can [perform](Content::perform).
#[derive(Debug, Clone)]
pub enum Action {
    EditName(String),
    EditId(String),
    OverviewContent(st_column::Action),
    EditECTS(String),
    EditVersion(String),
}

impl Content {
    /// Creates a new [Content] from a module.
    pub fn new(module: Module) -> Self {
        Self {
            module,
            overview_content: STContent::new(StorageTree::node((), [StorageTree::leaf(())].into()))
                .retract_on_select(),
            editable: None,
        }
    }

    /// Sets all the edits to a value.
    pub fn set_all_edits(self, value: bool) -> Self {
        Self {
            editable: Some(Editable::default().set_all(value)),
            ..self
        }
    }

    /// Performs an action
    pub fn perform(&mut self, action: Action) {
        macro_rules! set_editable_field {
            ($field:ident, $value:expr) => {{
                self.editable
                    .as_mut()
                    .map(|editable| editable.$field = $value);
            }};
        }

        match action {
            Action::EditName(str) => *self.get_mut_name() = str,
            Action::EditId(str) => *self.get_mut_id() = str,
            Action::OverviewContent(action) => self.overview_content.perform(action),
            Action::EditECTS(val) => {
                let parsed = val.parse();
                self.editable
                    .as_mut()
                    .map(|editable| editable.ects_string = val);
                match parsed {
                    Ok(val) => {
                        set_editable_field!(ects_error, None);
                        *self.get_mut_ects() = val
                    }
                    Err(err) => set_editable_field!(ects_error, Some(err)),
                }
            }
            Action::EditVersion(str) => *self.get_mut_version() = str,
        }
    }
}

impl<'a> ModuleDisplay<'a> {
    /// Creates a new [ModuleDisplay].
    pub fn new(content: &'a Content) -> Self {
        Self { content }
    }
}

impl<'a> From<ModuleDisplay<'a>> for Element<'a, Action> {
    fn from(value: ModuleDisplay<'a>) -> Self {
        macro_rules! texter {
            ($text:ident, $field:ident, $msg:expr) => {
                if value
                    .content
                    .editable
                    .as_ref()
                    .map(|x| x.$field)
                    .unwrap_or(false)
                {
                    <_ as Into<Element<'_, _>>>::into(
                        transparent_text_input(stringify!($field), &value.content.$field).on_input($msg),
                    )
                } else {
                    $text(&value.content.$field).into()
                }
            };
        }

        let content = value.content;

        // ==== BANNER ====

        let banner = row![
            texter!(bald_text, name, Action::EditName),
            horizontal_space(),
            row![text("(id: "), texter!(text, id, Action::EditId), text(")")]
                .align_y(Alignment::Center)
        ];

        // ==== OVERVIEW ====

        let overview = STColumn::new(
            &content.overview_content,
            Action::OverviewContent,
            // Header
            |_, _| {
                container("Overview")
                    .style(container::rounded_box)
                    .width(Fill)
                    .padding(PADDING)
                    .into()
            },
            |_, _| {
                column![
                    // ECTS
                    row![
                        bald_text("ECTS credits: "),
                        if let Some(editable) = &value.content.editable
                            && editable.ects
                        {
                            let editor = row![transparent_text_input("ECTS", &editable.ects_string)
                                .on_input(Action::EditECTS)]
                                .align_y(Center)
                                .spacing(PADDING);
                            match &editable.ects_error {
                                None => editor.into(),
                                Some(err) => container(editor.push(text(err.to_string())).push(horizontal_space().width(Shrink)))
                                    .style(|_| container::background(ERROR_COLOR))
                                    .into(),
                            }
                        } else {
                            <_ as Into<Element<'_, _>>>::into(text(&value.content.ects))
                        }
                    ]
                    .align_y(Center),
                    // Version
                    row![
                        bald_text("Version: "),
                        texter!(text, version, Action::EditVersion)
                    ]
                    .align_y(Center)
                ]
                .into()
            },
        )
        .space(MENU_OFFSET)
        .icons_default(PADDING);

        column![banner, horizontal_rule(PADDING), overview].into()
    }
}

impl AsRef<Module> for Content {
    fn as_ref(&self) -> &Module {
        &**self
    }
}
impl AsMut<Module> for Content {
    fn as_mut(&mut self) -> &mut Module {
        &mut **self
    }
}
impl Borrow<Module> for Content {
    fn borrow(&self) -> &Module {
        &**self
    }
}
impl BorrowMut<Module> for Content {
    fn borrow_mut(&mut self) -> &mut Module {
        &mut **self
    }
}

impl Deref for Content {
    type Target = Module;

    fn deref(&self) -> &Self::Target {
        &self.module
    }
}

impl DerefMut for Content {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.module
    }
}
