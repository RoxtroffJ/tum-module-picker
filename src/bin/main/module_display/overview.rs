//! The overview section of the module display.

use std::ops::{Deref, DerefMut};

use iced::{
    Alignment::Center,
    Element, Task,
    advanced::graphics::futures::MaybeSend,
    widget::{row, text, text_editor},
};
use tum_module_picker::{
    displayable_option::{DisplayOption, opt_to_string},
    module::{Module, semester::Semester},
    storage_tree::{
        StorageTree,
        column::{Action as STAction, Content as STContent},
    },
};

use crate::{
    bald_text, content_column,
    module_display::{Edit, Resetable, section::*},
    non_str_texter, set_non_str_field, set_str_field, texter,
};

/// Content of the [Overview].
#[derive(Debug)]
pub struct Content {
    overview_content: STContent<(), Option<Editable>>,
}

mod editable;
use editable::*;

/// The actions performed by the [perform](Content::perform) function.
#[derive(Debug, Clone)]
pub enum Action {
    StorageTree(STAction),

    ECTS(String),
    Version(String),
    ValidFrom(String),
    ValidUntil(String),
    ValidFromParsed(Option<Semester>),
    ValidUntilParsed(Option<Semester>),
    Responsible(String),
    Organisation(String),
    Note(String),

    Editor(text_editor::Action),
}

impl Content {
    /// Creates a new [Content].
    pub fn new() -> Self {
        Self {
            overview_content: STContent::new(StorageTree::node(
                (),
                [StorageTree::leaf(None)].into(),
            ))
            .retract_on_select(),
        }
    }

    /// Same as self but with all the edits to a value.
    pub fn with_all_edits(mut self, value: bool, module: &Module) -> Self {
        self.set_all_edits(value, module);
        self
    }

    /// Sets all the edits to a value.
    pub fn set_all_edits(&mut self, value: bool, module: &Module) {
        self.mut_leaf_iter()
            .map(|editable| {
                if editable.as_ref().is_none() && !value {
                    return;
                }

                editable.get_or_insert(Editable::new(module)).set_all(value)
            })
            .collect()
    }

    /// Views the [Content] by turning it into an element.
    pub fn view<'a>(&'a self, module: &'a Module) -> Element<'a, Action> {
        section(
            |_| "Overview",
            move |editable| {
                content_column![
                    // ECTS
                    row![
                        bald_text("ECTS credits: "),
                        non_str_texter!(
                            module,
                            editable,
                            text,
                            ects,
                            ects_string,
                            ects_error,
                            Action::ECTS
                        )
                    ]
                    .align_y(Center),
                    // Version
                    row![
                        bald_text("Version: "),
                        texter!(module, editable, text, version, Action::Version)
                    ]
                    .align_y(Center),
                    // Valid
                    row![bald_text("Valid: "), {
                        if let Some(editable) = editable
                            && (editable.valid_from || editable.valid_until)
                        {
                            let opt_text = |x: &Option<Semester>| text(opt_to_string(x));
                            content_column![]
                                .push_maybe(if editable.valid_from {
                                    Some(
                                        row![
                                            text("From: "),
                                            non_str_texter!(
                                                module,
                                                Some(editable),
                                                opt_text,
                                                valid_from,
                                                valid_from_string,
                                                valid_from_error,
                                                Action::ValidFrom
                                            )
                                        ]
                                        .align_y(Center),
                                    )
                                } else {
                                    None
                                })
                                .push_maybe(if editable.valid_until {
                                    Some(
                                        row![
                                            text("Until: "),
                                            non_str_texter!(
                                                module,
                                                Some(editable),
                                                opt_text,
                                                valid_until,
                                                valid_until_string,
                                                valid_until_error,
                                                Action::ValidUntil
                                            )
                                        ]
                                        .align_y(Center),
                                    )
                                } else {
                                    None
                                })
                                .into()
                        } else {
                            <_ as Into<Element<'_, _>>>::into(
                                if module.valid_from.is_none() && module.valid_until.is_none() {
                                    row!["-"]
                                } else {
                                    row![text("Valid")]
                                        .push_maybe(module.valid_from.as_ref().map(|semester| {
                                            row![text(" from "), text(semester.to_string())]
                                        }))
                                        .push_maybe(module.valid_until.as_ref().map(|semester| {
                                            row![text(" until "), text(semester.to_string())]
                                        }))
                                },
                            )
                        }
                    }],
                    // Responsible for module
                    row![
                        bald_text("Responsible for module: "),
                        texter!(module, editable, text, responsible, Action::Responsible)
                    ]
                    .align_y(Center),
                    // Organisation
                    row![
                        bald_text("Organisation: "),
                        texter!(module, editable, text, organisation, Action::Organisation)
                    ]
                    .align_y(Center),
                    // Note
                    row![
                        bald_text("Note: "),
                        texter!(module, editable, text, note, Action::Note)
                    ]
                    .align_y(Center),
                ]
                .into()
            },
            |editable| {
                editable
                    .as_ref()
                    .and_then(|e| edit_text_editor(e, &e.overview_content, Action::Editor))
            },
            |editable| editable.as_ref().map(Edit::has_one_error).unwrap_or(false),
            self,
            Action::StorageTree,
        )
    }

    /// Performs the given [Action].
    pub fn perform<Message: MaybeSend + 'static>(
        &mut self,
        action: Action,
        module: &mut Module,

        on_action: impl Fn(Action) -> Message,
        name_edit: impl Fn(String) -> Message,
        id_edit: impl Fn(String) -> Message,
    ) -> Task<Message> {
        let editable = self.mut_leaf_iter().next().map(Option::as_mut).flatten();
        match action {
            Action::StorageTree(action) => self.overview_content.perform(action),

            Action::ECTS(str) => set_non_str_field!(
                module,
                editable,
                str.parse(),
                str,
                ects,
                ects_string,
                ects_error
            ),
            Action::Version(str) => set_str_field!(module, editable, str, version),
            Action::ValidFrom(str) => set_non_str_field!(
                module,
                editable,
                str.parse().map(<DisplayOption<_> as Into<_>>::into),
                str,
                valid_from,
                valid_from_string,
                valid_from_error
            ),
            Action::ValidUntil(str) => set_non_str_field!(
                module,
                editable,
                str.parse().map(<DisplayOption<_> as Into<_>>::into),
                str,
                valid_until,
                valid_until_string,
                valid_until_error
            ),
            Action::ValidFromParsed(semester) => {
                let displ: DisplayOption<_> = semester.into();
                let str = displ.to_string();
                let value = displ.into();

                set_non_str_field!(
                    module,
                    editable,
                    Ok(value),
                    str,
                    valid_from,
                    valid_from_string,
                    valid_from_error
                )
            }
            Action::ValidUntilParsed(semester) => {
                let displ: DisplayOption<_> = semester.into();
                let str = displ.to_string();
                let value = displ.into();

                set_non_str_field!(
                    module,
                    editable,
                    Ok(value),
                    str,
                    valid_until,
                    valid_until_string,
                    valid_until_error
                )
            }
            Action::Responsible(str) => set_str_field!(module, editable, str, responsible),
            Action::Organisation(str) => set_str_field!(module, editable, str, organisation),
            Action::Note(str) => set_str_field!(module, editable, str, note),
            Action::Editor(action) => {
                return editor_perform::perform(action, editable, &on_action, &name_edit, &id_edit);
            }
        };

        Task::none()
    }

    /// Resets the fields to match the given module.
    pub fn reset(&mut self, module: &Module) {
        self.overview_content
            .mut_leaf_iter()
            .map(|x| x.as_mut().map(|x| x.reset(module)).unwrap_or_default())
            .collect()
    }

    /// Expands or collapses all the expandable fields.
    pub fn expand_all(&mut self, value: bool) {
        self.overview_content.expand_all(value);
    }
}

mod editor_perform;

impl AsRef<STContent<(), Option<Editable>>> for Content {
    fn as_ref(&self) -> &STContent<(), Option<Editable>> {
        &**self
    }
}

impl AsMut<STContent<(), Option<Editable>>> for Content {
    fn as_mut(&mut self) -> &mut STContent<(), Option<Editable>> {
        &mut **self
    }
}

impl Deref for Content {
    type Target = STContent<(), Option<Editable>>;

    fn deref(&self) -> &Self::Target {
        &self.overview_content
    }
}

impl DerefMut for Content {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.overview_content
    }
}
