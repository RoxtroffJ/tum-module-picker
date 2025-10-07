/// Either text or text input.
///
/// # Arguments:
/// - module: The [Module](tum_module_picker::module::module::Module) being displayed.
/// - content: struct holding the [Editable] field, as well as capable of dereferencing to something holding the field being displayed.
/// - text: identifyer to the function used to create the text widget. ([text()], [bald_text], ...).
/// - field: the field being displayed.
/// - msg: Message produced when the text_input is modified.
#[macro_export]
macro_rules! texter {
    ($module:expr, $editable:expr, $text:ident, $field:ident, $msg:expr) => {{
        use crate::transparent_text_input;
        if $editable
            .as_ref()
            .map(|x| x.$field)
            .unwrap_or(false)
        {
            <_ as Into<Element<'_, _>>>::into(
                transparent_text_input(stringify!($field), &$module.$field).on_input($msg),
            )
        } else {
            $text(&$module.$field).into()
        }
    }};
}

#[macro_export]
macro_rules! editor_texter {
    ($module:expr, $editable:expr, $text:ident, $field:ident, $editor_field:ident, $msg:expr) => {{
        use crate::transparent_text_editor;
        if let Some(editable) = $editable && editable.$field
        {
            <_ as Into<Element<'_, _>>>::into(
                transparent_text_editor(stringify!($field), &editable.$editor_field).on_action($msg),
            )
        } else {
            $text(&$module.$field).into()
        }
    }};
}

/// Same as [texter] but for non string fields.
///
/// Capable of displaying errors if the parsing is unsuccessful.
///
/// # Arguments:
/// - module: The [Module] being displayed.
/// - content: struct holding the [Editable] field, as well as capable of dereferencing to something holding the field being displayed.
/// - text: identifyer to the function used to create the text widget. ([text()], [bald_text], ...).
/// - field: the field being displayed.
/// - str_field: the field holding the display string.
/// - err_field: the field holding the error option.
/// - msg: Message produced when the text_input is modified.
#[macro_export]
macro_rules! non_str_texter {
    ($module:expr, $editable:expr, $text:expr, $field:ident, $str_field:ident, $err_field:ident, $msg:expr) => {{
        use crate::*;
        use iced::{widget::{container, horizontal_space}, Length::*};
        if let Some(editable) = $editable
            && editable.$field
        {
            let editor = row![
                transparent_text_input(stringify!($field), &editable.$str_field).on_input($msg)
            ]
            .align_y(iced::Alignment::Center)
            .spacing(PADDING);
            match &editable.$err_field {
                None => editor.into(),
                Some(err) => container(
                    editor
                        .push(text(err.to_string()))
                        .push(horizontal_space().width(Shrink)),
                )
                .style(backgrounded(container::transparent, ERROR_COLOR))
                .into(),
            }
        } else {
            <_ as Into<Element<'_, _>>>::into($text(&$module.$field))
        }
    }};
}

/// Same as [column!] but formatted for the content of a section.
#[macro_export]
macro_rules! content_column {
    ($($x:expr),* $(,)?) => {
        {
            use iced::widget::column;
            use crate::PADDING;
            column![$($x),*].spacing(PADDING / 2)
        }
    };
}

pub mod section;