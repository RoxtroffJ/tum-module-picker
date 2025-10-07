use std::num::ParseIntError;

use iced::widget::text_editor;
use tum_module_picker::{displayable_option::opt_to_string, module::{semester, ECTS}};

use crate::editable_maker;

editable_maker! {
    pub,
    Editable,
    ects, version, valid_from, valid_until, responsible, organisation, note ;

    ects_string {ects ECTS::to_string} ects_error ParseIntError,
    valid_from_string {valid_from opt_to_string} valid_from_error semester::ParseError,
    valid_until_string {valid_until opt_to_string} valid_until_error semester::ParseError ;

    overview_content text_editor::Content = text_editor::Content::new()
}
