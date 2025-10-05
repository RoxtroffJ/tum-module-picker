use super::*;
use tum_module_picker::storage_tree::column::{Action as STAction, Content as STContent};

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
    ($module:expr, $editable:expr, $text:ident, $field:ident, $str_field:ident, $err_field:ident, $msg:expr) => {{
        use crate::*;
        use iced::{widget::{container, horizontal_space}, Length::*};
        if let Some(editable) = $editable
            && editable.$field
        {
            let editor = row![
                transparent_text_input(stringify!($field), &editable.$str_field).on_input($msg)
            ]
            .align_y(Center)
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

/// Section header with given label, and indication of error.
pub fn section_header<'a, Action: 'a>(label: &'a str, is_error: bool) -> Element<'a, Action> {
    let mut result = container(label).width(Fill).padding(PADDING);

    if is_error {
        result = result.style(backgrounded(container::rounded_box, ERROR_COLOR))
    } else {
        result = result.style(container::rounded_box)
    }

    result.into()
}

/// A collapsable section build from a [storage_tree::Content](STContent) that has a single node with a single child.
pub fn section<'a, Action: Clone + 'a, K, T: 'a>(
    label: &'a str,
    content_builder: impl Fn(&'a T) -> Element<'a, Action> + 'a,
    is_error: bool,
    is_editable: Option<(
        &'a text_editor::Content,
        impl Fn(text_editor::Action) -> Action + 'a,
    )>,
    st_content: &'a STContent<K, T>,
    message: impl Fn(STAction) -> Action + 'a,
) -> Element<'a, Action> {
    let mut result = row![
        container(
            STColumn::new(
                st_content,
                message,
                move |_, _| section_header(label, is_error),
                move |x, _| content_builder(x)
            )
            .space(MENU_OFFSET)
            .spacing(PADDING)
            .icons_default(PADDING)
        )
        .width(FillPortion(2))
    ]
    .height(Shrink)
    .spacing(PADDING);

    match is_editable {
        Some((content, message)) => {
            result = result.push(
                container(TextEditor::new(content).height(Fill).on_action(message))
                    .width(FillPortion(1)),
            )
        }
        None => (),
    };

    result.into()
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
