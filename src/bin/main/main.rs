use iced::advanced;
use iced::advanced::text::highlighter::PlainText;
use iced::widget::{text_input, TextInput, text_editor, TextEditor};
use iced::{Color, Theme};
use iced::{
    Element, Font, Task,
    widget::{
        Text, container, text,
        text::{IntoFragment},
    },
};
use iced_aw::iced_fonts::REQUIRED_FONT_BYTES;
use iced_fonts::NERD_FONT_BYTES;
use tum_module_picker::storage_tree::StorageTree;

use crate::module_tree::ModuleTree;

mod module_display;
mod module_tree;

pub const PADDING: u16 = 10;
pub const MENU_OFFSET: f32 = 20.;

struct App {
    module_tree: ModuleTree,
}

#[derive(Debug, Clone)]
enum Message {
    ModuleTree(module_tree::Message),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let module_tree = ModuleTree::new(StorageTree::node("Aerospace".into(), Vec::new()));
        (   
            Self {
                module_tree,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ModuleTree(message) => {
                self.module_tree.update(message).map(Message::ModuleTree)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(self.module_tree.view().map(Message::ModuleTree))
            .padding(PADDING)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application("Module Picker", App::update, App::view)
        .font(NERD_FONT_BYTES)
        .font(REQUIRED_FONT_BYTES)
        .run_with(App::new)
}

/// Same as [text], but the text is bald
pub fn bald_text<'a, Theme, Render>(t: impl IntoFragment<'a>) -> Text<'a, Theme, Render>
where
    Theme: text::Catalog + 'a,
    Render: advanced::text::Renderer,
    <Render as iced::advanced::text::Renderer>::Font: From<iced::Font>,
{
    text(t).font(Font {
        weight: iced::font::Weight::Bold,
        ..Font::DEFAULT
    })
}

/// Container style that is the same as the provided one except for the color of the background.
pub fn backgrounded(style: impl Fn(&Theme) -> container::Style, color: Color) -> impl Fn(&Theme) -> container::Style {
    move |theme| {
        container::Style {background: Some(color.into()), ..style(theme)}
    }
}

/// Transparent [TextInput]
pub fn transparent_text_input<'a, Message>(
    placeholder: &str,
    value: &str,
) -> TextInput<'a, Message>
where
    Message: Clone,
{
    text_input(placeholder, value).style(|theme, status| {
        text_input::Style { background: Color::TRANSPARENT.into(), ..text_input::default(theme, status)}
    })
}

/// Transparent [TextEditor]
pub fn transparent_text_editor<'a, Message>(
    placeholder: &'a str,
    content: &'a text_editor::Content,
) -> TextEditor<'a, PlainText, Message>
where
    Message: Clone,
{
    text_editor(content).placeholder(placeholder).style(|theme, status| {
        text_editor::Style { background: Color::TRANSPARENT.into(), ..text_editor::default(theme, status)}
    })
}

pub const ERROR_COLOR: Color = Color::from_rgba(1.0, 0., 0., 0.2);
pub const INACTIVE_COLOR: Color = Color::from_rgba(0.5, 0.5, 0.5, 0.2);