//! Allows to create [Daemon]s with multiple independant windows.
//!
//! When all the windows are closed, the app closes.
//! When a window that had spawned popups closes, the popups closes.

use std::{any::Any, collections::HashMap, fmt::Debug};

use iced::{
    advanced::graphics::core::Element, futures::{
        channel::mpsc::{self, SendError},
        sink::SinkExt,
    }, widget::horizontal_space, window::{self, Id, Settings}, Subscription, Task
};

/// The [WindowStack] type. Can be rendered in a [Daemon](iced::daemon::Daemon).
///
/// It allows widgets to easely create popup windows.
pub struct WindowStack<Theme, Renderer> {
    stack: HashMap<Id, (Box<dyn GenericWindow<Theme, Renderer>>, Vec<Id>)>,
}

/// [Action]s that can be performed by a [WindowStack].
pub struct Action<Theme, Renderer> {
    inner: InnerAction<Theme, Renderer>,
}

impl<Theme, Renderer> Debug for Action<Theme, Renderer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Action")
            .field("inner", &self.inner)
            .finish()
    }
}

enum InnerAction<Theme, Renderer> {
    /// Forwards a message to a window.
    Forward(Id, Box<dyn Message>),
    /// Adds a window to the stack, with given parent.
    Push(
        Box<dyn GenericWindow<Theme, Renderer>>,
        Tasker<Box<dyn Message>>,
        Settings,
        Id,
    ),
    /// Adds a root window to the stack.
    PushRoot(
        Box<dyn GenericWindow<Theme, Renderer>>,
        Tasker<Box<dyn Message>>,
        Settings,
    ),
    /// Removes a window.
    Pop(Id),
}

impl<Theme, Renderer> Debug for InnerAction<Theme, Renderer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forward(arg0, arg1) => f.debug_tuple("Forward").field(arg0).field(arg1).finish(),
            Self::Push(arg0, arg1, arg2, arg3) => f
                .debug_tuple("Push")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .field(arg3)
                .finish(),
            Self::PushRoot(arg0, arg1, arg2) => f
                .debug_tuple("PushRoot")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::Pop(arg0) => f.debug_tuple("Pop").field(arg0).finish(),
        }
    }
}

struct Tasker<T> {
    task: Task<T>,
}

impl<T> Debug for Tasker<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tasker").field("Task", &"").finish()
    }
}

impl<Theme, Renderer> From<Action<Theme, Renderer>> for InnerAction<Theme, Renderer> {
    fn from(value: Action<Theme, Renderer>) -> Self {
        value.inner
    }
}

impl<Theme, Renderer> From<InnerAction<Theme, Renderer>> for Action<Theme, Renderer> {
    fn from(value: InnerAction<Theme, Renderer>) -> Self {
        Self { inner: value }
    }
}

impl<Theme: 'static, Renderer: iced::advanced::Renderer + 'static> WindowStack<Theme, Renderer> {
    /// Creates a new stack with the given window.
    ///
    /// The provided task will be run when the window is created.
    pub fn new<W: Window<Theme, Renderer> + 'static>(
        window: (W, Task<W::Message>),
        settings: Settings,
    ) -> (Self, Task<Action<Theme, Renderer>>) {
        let (window, task) = window;
        (
            Self {
                stack: HashMap::new(),
            },
            Task::done(
                InnerAction::PushRoot(
                    Box::new(window) as _,
                    Tasker {
                        task: task.map(|x| Box::new(x) as _),
                    },
                    settings,
                )
                .into(),
            ),
        )
    }

    /// Updates the stack.
    pub fn update(&mut self, action: Action<Theme, Renderer>) -> Task<Action<Theme, Renderer>> {
        match action.into() {
            InnerAction::Forward(id, message) => {
                let messager = Messager::new();
                let updater = self
                    .stack
                    .get_mut(&id)
                    .map(|(window, _)| window.update(message, messager.sender))
                    .unwrap_or(Task::none())
                    .map(move |x| InnerAction::Forward(id, x).into());
                let pusher = Task::run(messager.reciever, move |((state, task), settings)| {
                    InnerAction::Push(state, task, settings, id).into()
                });
                Task::batch(vec![updater, pusher])
            }
            InnerAction::Push(window, tasker, settings, parent) => {
                if let Some((_, parent_vec)) = self.stack.get_mut(&parent) {
                    let (id, open) = window::open(settings);
                    parent_vec.push(id);
                    self.stack.insert(id, (window, Vec::new()));
                    Task::batch(vec![
                        open.discard(),
                        tasker.task.map(move |x| InnerAction::Forward(id, x).into()),
                    ])
                } else {
                    Task::none()
                }
            }
            InnerAction::PushRoot(window, tasker, settings) => {
                let (id, open) = window::open(settings);
                self.stack.insert(id, (window, Vec::new()));
                Task::batch(vec![
                    open.discard(),
                    tasker.task.map(move |x| InnerAction::Forward(id, x).into()),
                ])
            }
            InnerAction::Pop(id) => {
                if let Some((_, vec)) = self.stack.remove(&id) {
                    if self.stack.is_empty() {
                        return iced::exit();
                    }
                    Task::batch(
                        vec.into_iter()
                            .map(|x| Task::done(InnerAction::Pop(x).into())),
                    )
                } else {
                    Task::none()
                }
            }
        }
    }

    /// View a window of the stack
    pub fn view(&self, window: Id) -> Element<'_, Action<Theme, Renderer>, Theme, Renderer> {
        match self.stack.get(&window) {
            Some((state, _)) => state
                .view()
                .map(move |x| InnerAction::Forward(window, x).into()),
            None => horizontal_space().into(),
        }
    }

    /// Title of the windows of the stack
    pub fn title(&self, window: Id) -> String {
        self.stack
            .get(&window)
            .map(|(x, _)| x.title())
            .unwrap_or_default()
    }

    /// Subscription logic of the stack
    pub fn subscription(&self) -> Subscription<Action<Theme, Renderer>> {
        window::close_events().map(|id| InnerAction::Pop(id).into())
    }
}

/// Creates a deamon for a [WindowStack].
#[macro_export]
macro_rules! window_stack_deamon {
    () => {
        iced::daemon(WindowStack::title, WindowStack::update, WindowStack::view)
        .subscription(WindowStack::subscription)
    };
}

type WindowSend<Theme, Renderer> = (
    (
        Box<dyn GenericWindow<Theme, Renderer>>,
        Tasker<Box<dyn Message>>,
    ),
    Settings,
);
type RX<Theme, Renderer> = mpsc::Receiver<WindowSend<Theme, Renderer>>;

struct Messager<Theme, Renderer> {
    reciever: RX<Theme, Renderer>,
    sender: PopupMaker<Theme, Renderer>,
}

impl<Theme, Renderer> Messager<Theme, Renderer> {
    fn new() -> Self {
        let (sx, rx) = mpsc::channel(100);
        Self {
            reciever: rx,
            sender: PopupMaker { sender: sx },
        }
    }
}

/// Allows to create popup windows.
#[derive(Debug)]
pub struct PopupMaker<Theme = iced::Theme, Renderer = iced::Renderer> {
    sender: mpsc::Sender<WindowSend<Theme, Renderer>>,
}

impl<Theme, Renderer> Clone for PopupMaker<Theme, Renderer> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<Theme: 'static, Renderer: iced::advanced::Renderer + 'static> PopupMaker<Theme, Renderer> {
    /// Creates a new popup from the provided state - task pair and [window::Settings].
    pub fn popup<W: Window<Theme, Renderer> + 'static>(
        mut self,
        state: (W, Task<W::Message>),
        settings: Settings,
    ) -> Task<Result<(), SendError>> {
        let (state, task) = state;
        let foo = || async move {
            self.sender
                .send((
                    (
                        Box::new(state) as _,
                        Tasker {
                            task: task.map(|x| Box::new(x) as _),
                        },
                    ),
                    settings,
                ))
                .await
        };

        Task::future(foo())
    }
}

/// A window that can be put in a [WindowStack].
trait GenericWindow<Theme, Renderer>: Debug + Send {
    /// Generic view logic.
    fn view<'a>(&'a self) -> Element<'a, Box<dyn Message>, Theme, Renderer>
    where
        Theme: 'a,
        Renderer: 'a;

    /// Generic update logic.
    fn update(
        &mut self,
        message: Box<dyn Any>,
        popup_maker: PopupMaker<Theme, Renderer>,
    ) -> Task<Box<dyn Message>>;

    /// Returns the title of the window.
    fn title(&self) -> String;
}

/// The requirement for a type to be a Message
pub trait Message: Any + Send + Debug {}
impl<T: Any + Send + Debug> Message for T {}

/// Trait to make it possible to run a window in a [WindowStack].
pub trait Window<Theme = iced::Theme, Renderer = iced::Renderer>: Send + Debug + Sized {
    /// Type of messages
    type Message: Message;

    /// The update logic
    fn update(
        &mut self,
        message: Self::Message,
        popup_maker: PopupMaker<Theme, Renderer>,
    ) -> impl Into<Task<Self::Message>>;

    /// The update function
    fn view(&self) -> impl Into<Element<'_, Self::Message, Theme, Renderer>>;

    /// Sets the title of the application.
    fn title(&self) -> String;
}

impl<Theme, Renderer: iced::advanced::Renderer, T: Window<Theme, Renderer>>
    GenericWindow<Theme, Renderer> for T
{
    fn view<'a>(&'a self) -> Element<'a, Box<dyn Message>, Theme, Renderer>
    where
        Theme: 'a,
        Renderer: 'a,
    {
        self.view().into().map(|x| Box::new(x) as Box<dyn Message>)
    }

    fn update(
        &mut self,
        message: Box<dyn Any>,
        popup_maker: PopupMaker<Theme, Renderer>,
    ) -> Task<Box<(dyn Message)>> {
        match message.downcast() {
            Ok(message) => self
                .update(*message, popup_maker)
                .into()
                .map(|t| Box::new(t) as Box<dyn Message>),
            Err(_) => Task::none(),
        }
    }

    fn title(&self) -> String {
        self.title()
    }
}
