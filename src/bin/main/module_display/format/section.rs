//! Collapsable titled sections that can be valid or not.

use iced::{
    widget::{container, row, text, text_editor, TextEditor}, Element, Length::*
};
use tum_module_picker::storage_tree::column::{Action as STAction, Column as STColumn, Content as STContent};

use crate::{ERROR_COLOR, MENU_OFFSET, PADDING, backgrounded, module_display::Edit};

/// Section header with given label, and indication of error.
pub fn section_header<'a, Action: 'a>(label: impl ToString, is_error: bool) -> Element<'a, Action> {
    let mut result = container(text(label.to_string())).width(Fill).padding(PADDING);

    if is_error {
        result = result.style(backgrounded(container::rounded_box, ERROR_COLOR))
    } else {
        result = result.style(container::rounded_box)
    }

    result.into()
}

pub fn edit_text_editor<'a, Action: 'a, Editable: Edit>(
    editable: &Editable,
    content: &'a text_editor::Content,
    message: impl Fn(text_editor::Action) -> Action + 'a,
) -> Option<Element<'a, Action>> {
    if editable.has_one_editable() {
    Some(container(TextEditor::new(content).height(Shrink).on_action(message))
        .width(FillPortion(1))
        .into())
    } else {
        None
    }
}

/// A collapsable section build from a [storage_tree::column::Content](STContent).
pub fn section<'a, Action: Clone + 'a, K, T: 'a, Label: ToString>(
    label_builder: impl Fn(&'a K) -> Label + 'a,
    content_builder: impl Fn(&'a T) -> Element<'a, Action> + 'a,

    is_editable: impl Fn(&'a T) -> Option<Element<'a, Action>> + 'a,
    is_error: impl Fn(&'a T) -> bool + 'a,

    st_content: &'a STContent<K, T>,
    message: impl Fn(STAction) -> Action + 'a,
) -> Element<'a, Action> {
    STColumn::new(
        st_content,
        message,
        move |k, _| {
            section_header(
                label_builder(k.get_key()),
                k.get_children()
                    .iter()
                    .map(|x| x.leaf_iter())
                    .flatten()
                    .fold(false, |a, b| a || is_error(b)),
            )
        },
        move |x, _| {
            row![container(content_builder(x)).width(FillPortion(2))]
                .height(Shrink)
                .spacing(PADDING)
                .push_maybe(is_editable(x))
                .into()
        },
    )
    .space(MENU_OFFSET)
    .spacing(PADDING/2)
    .icons_default(PADDING)
    .into()
}
