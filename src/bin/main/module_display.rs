use crate::*;

use iced::{
    Alignment::{self},
    Element,
    widget::{column, horizontal_rule, horizontal_space, row},
};

mod editable;
pub use editable::*;

mod content;
pub use content::{*, Content};

/// Struct that implements [Into<Element>].
pub struct ModuleDisplay<'a> {
    content: &'a Content,
}

impl<'a> ModuleDisplay<'a> {
    /// Creates a new [ModuleDisplay].
    pub fn new(content: &'a Content) -> Self {
        Self { content }
    }
}

mod format;
pub use format::*;


mod overview;

mod description;

impl<'a> From<ModuleDisplay<'a>> for Element<'a, Action> {
    fn from(value: ModuleDisplay<'a>) -> Self {
        let content = value.content;

        // ==== BANNER ====

        let banner = row![
            texter!(content, content.editable, bald_text, name, Action::Name),
            horizontal_space(),
            row![
                text("(id: "),
                texter!(content, content.editable, text, id, Action::Id),
                text(")")
            ]
            .align_y(Alignment::Center)
        ];

        // ==== OVERVIEW ====

        let overview = content.overview_content.view(content).map(Action::Overview);
        let description = content.description_content.view(content).map(Action::Description);

        column![banner, horizontal_rule(PADDING), overview, description].into()
    }
}
