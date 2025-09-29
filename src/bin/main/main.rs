use iced::{widget::{center, container, text, Button}, Element, Task};
use iced_aw::iced_fonts::REQUIRED_FONT_BYTES;
use iced_fonts::NERD_FONT_BYTES;
use tum_module_picker::{module::Module, storage_tree::{self, Path, StorageTree}};

use crate::module_tree::{folder_to_element, ModuleTree};

mod module_tree;

const PADDING: u16 = 10;
const MENU_OFFSET: f32 = 6.;

struct App {
    module_tree: ModuleTree
}

#[derive(Debug, Clone)]
enum Message {
    ModuleTree(module_tree::Message)
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                module_tree: ModuleTree::new(StorageTree::node("Aerospace".into(), Vec::new()))
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ModuleTree(message) => self.module_tree.update(message).map(Message::ModuleTree),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(self.module_tree.view().map(Message::ModuleTree)).padding(PADDING).into()
    }
}

fn main() -> iced::Result {
    iced::application("Module Picker", App::update, App::view)
        .font(NERD_FONT_BYTES)
        .font(REQUIRED_FONT_BYTES)
        .run_with(App::new)
}
