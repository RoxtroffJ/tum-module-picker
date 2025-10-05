use std::num::ParseIntError;

use iced::widget::text_editor;
use tum_module_picker::{displayable_option::{opt_to_string, DisplayOption}, module::{semester, Module}};

use crate::editable_maker;

editable_maker! {
    pub(super),
    Editable,
    ects, version, valid_from, valid_until, responsible, organisation, note ;

    ects_string ects_error ParseIntError,
    valid_from_string valid_from_error semester::ParseError,
    valid_until_string valid_until_error semester::ParseError ;

    overview_content text_editor::Content
}

impl Editable {
    pub fn new(module: &Module) -> Self {
        Self {
            ects: false,
            version: false,
            valid_from: false,
            valid_until: false,
            responsible: false,
            organisation: false,
            note: false,
            
            ects_string: module.ects.to_string(),
            ects_error: None,

            valid_from_string: opt_to_string(&module.valid_from),
            valid_from_error: None,

            valid_until_string: opt_to_string(&module.valid_until),
            valid_until_error: None,

            overview_content: text_editor::Content::new(),
        }
    }
}
