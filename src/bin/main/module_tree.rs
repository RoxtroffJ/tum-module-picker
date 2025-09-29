//! Helpers for the [ModuleTree] widget.
use super::*;
use iced::{
    widget::{
        button, center, column, container::background, horizontal_space, mouse_area, opaque, row, stack, text_input
    }, Color, Element, Length::{Fill, Shrink}
};
use iced_aw::ContextMenu;
use iced_fonts::{Nerd, nerd::to_text};
use tum_module_picker::storage_tree::{
    Node, Path,
    column::{Action, Content, MetaKey, NodeState},
};

pub struct ModuleTree {
    content: Content<String, Module>,
    new_menu_name: String,
    path: Path,
    overlay: Overlay,
}

#[derive(Debug, Clone)]
enum Overlay {
    None,
    Folder,
    Module,
}

#[derive(Debug, Clone)]
pub enum Message {
    ModuleTree(storage_tree::column::Action),
    AddFolder(Path),
    EditAddFolder(String),
    AddModule(Module, Path),
    NewFolderPressed(Path),
    NewModulePressed(Path),
    OverlayQuit,
}

impl ModuleTree {
    pub fn new(tree: StorageTree<String, Module>) -> Self {
        Self {
            content: Content::new(tree),
            new_menu_name: "".into(),
            path: Path::default(),
            overlay: Overlay::None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ModuleTree(action) => self.content.perform(action),
            Message::AddFolder(path) => {
                let name = std::mem::take(&mut self.new_menu_name);
                let new_folder =
                    StorageTree::node(MetaKey::new(name, NodeState::default()), Vec::new());
                self.content.add(new_folder, &path);
                return Task::done(Message::ModuleTree(Action::Expand(path)));
            }
            Message::EditAddFolder(text) => self.new_menu_name = text,
            Message::AddModule(_, _) => todo!(),
            Message::NewFolderPressed(path) => {
                self.path = path;
                self.overlay = Overlay::Folder
            }
            Message::NewModulePressed(path) => {
                self.path = path;
                self.overlay = Overlay::Module
            }
            Message::OverlayQuit => self.overlay = Overlay::None,
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let underlay = container(storage_tree::column::Column::new(
            &self.content,
            Message::ModuleTree,
            |name, path| folder_to_element(name, path),
            |_, _| text("Foo").into(),
        )
        .space(20.)
        .icon(
            || {
                row![
                    to_text(Nerd::TriangleRight),
                    horizontal_space().width(PADDING)
                ]
                .into()
            },
            || {
                row![
                    to_text(Nerd::TriangleDown),
                    horizontal_space().width(PADDING)
                ]
                .into()
            },
        ))
        .width(Fill)
        .height(Fill)
        .into();

        let overlay = match self.overlay {
            Overlay::None => return underlay,
            Overlay::Folder => container(column![
                text("New folder name:"),
                text_input("Type here...", &self.new_menu_name).on_input(Message::EditAddFolder).on_submit(Message::AddFolder(self.path.clone())),
            ].width(Shrink)),
            Overlay::Module => todo!(),
        }.style(|theme: &iced::Theme| background(theme.palette().background)).padding(PADDING);

        modal(underlay, overlay, Message::OverlayQuit)
    }
}

pub fn folder_to_element<'a, M>(
    node: &'a Node<MetaKey<String, NodeState>, M>,
    path: Path,
) -> Element<'a, Message> {
    let name = &**node.get_key();
    let label = text(name).center();

    ContextMenu::new(label, move || {
        container(column![
            button("New folder")
                .style(button::text)
                .on_press(Message::NewFolderPressed(path.clone())),
            button("New module")
                .style(button::text)
                .on_press(Message::NewModulePressed(path.clone())),
        ])
        .style(container::rounded_box)
        .into()
    })
    .into()
}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.1,
                            ..if theme.extended_palette().is_dark {Color::WHITE} else {Color::BLACK}
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}
