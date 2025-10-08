//! Helpers for the [ModuleTree] widget.

use crate::module_display::ModuleDisplay;

use super::*;
use iced::{
    futures::{channel::mpsc, SinkExt}, widget::{
        button, center, column, container::background, horizontal_rule, mouse_area, opaque,
        scrollable, stack, text,
    }, Element, Length::{Fill, Shrink}
};
use iced_aw::ContextMenu;
use tum_module_picker::{
    module::Module,
    storage_tree::{
        self, Node, Path,
        column::{Action, Content, MetaKey, NodeState},
    },
    window_stack::PopupMaker,
};

#[derive(Debug)]
pub struct ModuleTree {
    content: Content<String, Module>,

    path: Path,
    overlay: Overlay,

    new_folder_name: String,
    //new_module_content: module_display::Content,
}

#[derive(Debug, Clone)]
enum Overlay {
    None,
    Folder,
    //Module,
}

#[derive(Debug, Clone)]
pub enum Message {
    ModuleTree(storage_tree::column::Action),
    AddFolder(Path),
    EditAddFolder(String),
    AddModule(Path, Module),
    NewFolderPressed(Path),
    NewModulePressed(Path),
    OverlayQuit,
    //ModuleBuilder(module_display::Action),
}

impl ModuleTree {
    pub fn new(tree: StorageTree<String, Module>) -> Self {
        Self {
            content: Content::new(tree),
            new_folder_name: "".into(),
            path: Path::default(),
            overlay: Overlay::None,
            //new_module_content,
        }
    }

    pub fn update(&mut self, message: Message, popup: PopupMaker) -> Task<Message> {
        match message {
            Message::ModuleTree(action) => self.content.perform(action),
            Message::AddFolder(path) => {
                let name = std::mem::take(&mut self.new_folder_name);
                let new_folder =
                    StorageTree::node(MetaKey::new(name, NodeState::default()), Vec::new());
                self.content.add(new_folder, &path);
                return Task::done(Message::ModuleTree(Action::Expand(path)));
            }
            Message::EditAddFolder(text) => self.new_folder_name = text,
            Message::AddModule(_,_) => (), //self.new_module_content.set_all_edits(false),
            Message::NewFolderPressed(path) => {
                self.path = path;
                self.overlay = Overlay::Folder
            }
            Message::NewModulePressed(path) => {
                //self.overlay = Overlay::Module
                let (tx, rx) = mpsc::channel(1);
                return Task::batch(vec![popup
                    .popup(NewModulePopup::new(tx), Settings {
                        level: iced::window::Level::AlwaysOnTop,
                        ..Settings::default()
                    })
                    .discard(),
                    Task::run(rx, move |m| Message::AddModule(path.clone(), m))]);
            }
            Message::OverlayQuit => self.overlay = Overlay::None,
            //Message::ModuleBuilder(action) => return self.new_module_content.perform(action).map(Message::ModuleBuilder),
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let underlay = container(
            storage_tree::column::Column::new(
                &self.content,
                Message::ModuleTree,
                |name, path| folder_to_element(name, path),
                |_, _| text("Foo").into(),
            )
            .space(MENU_OFFSET)
            .icons_default(PADDING),
        )
        .width(Fill)
        .height(Fill)
        .into();

        let overlay = match self.overlay {
            Overlay::None => return underlay,
            Overlay::Folder => container(
                column![
                    text("New folder name:"),
                    transparent_text_input("Type here...", &self.new_folder_name)
                        .on_input(Message::EditAddFolder)
                        .on_submit(Message::AddFolder(self.path.clone())),
                ]
                .width(Shrink),
            ),
            // Overlay::Module => {
            //     let module_display: Element<'_, _> =
            //         ModuleDisplay::new(&self.new_module_content).into();
            //     container(
            //         scrollable(column![
            //             module_display.map(Message::ModuleBuilder),
            //             horizontal_rule(PADDING),
            //             button(bald_text("Create Module").width(Fill).center())
            //                 .on_press(Message::AddModule(self.path.clone()))
            //                 .style(button::success)
            //         ]).spacing(PADDING),
            //     )
            // }
        }
        .style(|theme: &iced::Theme| background(theme.palette().background))
        .padding(PADDING);

        modal(underlay, overlay, Message::OverlayQuit)
    }
}

#[derive(Debug)]
struct NewModulePopup {
    content: module_display::Content,
    tx: mpsc::Sender<Module>,
}

#[derive(Debug, Clone)]
enum NewModulePopupMsg {
    ModuleDisplay(module_display::Action),
    Done,
}

impl NewModulePopup {
    fn new(sx: mpsc::Sender<Module>) -> (Self, Task<NewModulePopupMsg>) {
        let mut content = module_display::Content::new(Module::default()).with_all_edits(true);
        content.expand_all(true);
        (Self { content, tx: sx }, Task::none())
    }
}

impl Window for NewModulePopup {
    type Message = NewModulePopupMsg;

    fn update(
        &mut self,
        message: Self::Message,
        _popup_maker: PopupMaker,
    ) -> impl Into<Task<Self::Message>> {
        match message {
            NewModulePopupMsg::ModuleDisplay(action) => self
                .content
                .perform(action)
                .map(NewModulePopupMsg::ModuleDisplay),
            NewModulePopupMsg::Done => {
                let _ = self.tx.send(self.content.replace_module(Module::default()));
                Task::none()
            }
        }
    }

    fn view(&self) -> impl Into<Element<'_, Self::Message>> {
        center(column![
            scrollable(
                <_ as Into<Element<'_, _>>>::into(ModuleDisplay::new(&self.content))
                    .map(NewModulePopupMsg::ModuleDisplay)
            ).height(Fill)
            .spacing(PADDING),
            horizontal_rule(PADDING),
            button(bald_text("Create Module").width(Fill).center())
                .on_press(NewModulePopupMsg::Done)
                .style(button::success)
        ].height(Fill).width(Fill)).padding(PADDING)
    }

    fn title(&self) -> String {
        "New Module".to_string()
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
            mouse_area(center(opaque(content)).style(|_| background(INACTIVE_COLOR)))
                .on_press(on_blur)
        )
    ]
    .into()
}
