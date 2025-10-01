//! Text input that parses its string.
//! 
//! It can [Deref] to a [TextInput].

use std::{borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut}};

use iced::{widget::{
    text_input::{Catalog, Icon, Id, Status, Style, StyleFn}, TextInput
}, Element};
use iced::{Length, Padding, Pixels, alignment};
use iced::advanced::text;


/// A field that parses it's content.
pub struct ParsedInput<'a, T, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    text_input: TextInput<'a, InternalMessage, Theme, Renderer>,
    parser: Box<dyn Fn(String) -> T + 'a>,

    on_input: Option<Box<dyn Fn(T) -> Message + 'a>>,
    on_submit: Option<Message>,
    on_paste: Option<Box<dyn Fn(T) -> Message + 'a>>,
}

/// Internal messages used to communicate with the inner [TextInput] of a [ParsedInput].
#[derive(Debug, Clone)]
pub enum InternalMessage {
    /// Variant used on input.
    Input(String),
    /// Variant used on submit.
    Submit,
    /// Variant used on paste.
    Paste(String),
}

impl<'a, T, Message, Theme, Renderer> ParsedInput<'a, T, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    /// Creates a new [ParsedInput].
    pub fn new<F: Fn(String) -> T + 'a>(placeholder: &str, value: &str, parser: F) -> Self {
        Self {
            text_input: TextInput::new(placeholder, value),
            parser: Box::new(parser),

            on_input: None,
            on_submit: None,
            on_paste: None,
        }
    }

    /// Sets the [Id] of the internal [TextInput].
    pub fn id(self, id: impl Into<Id>) -> Self {
        Self {
            text_input: self.text_input.id(id),
            ..self
        }
    }

    /// Converts the [ParsedInput] into a secure password input.
    pub fn secure(self, is_secure: bool) -> Self {
        Self {
            text_input: self.text_input.secure(is_secure),
            ..self
        }
    }

    /// Sets the message that should be produced when some text is typed into
    /// the [ParsedInput].
    ///
    /// If this method is not called, the [ParsedInput] will be disabled.
    pub fn on_input(mut self, on_input: impl Fn(T) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self.text_input = self.text_input.on_input(InternalMessage::Input);
        self
    }

    /// Sets the message that should be produced when some text is typed into
    /// the [ParsedInput], if `Some`.
    ///
    /// If `None`, the [ParsedInput] will be disabled.
    pub fn on_input_maybe(
        mut self,
        on_input: Option<impl Fn(T) -> Message + 'a>,
    ) -> Self {
        self.text_input = self.text_input.on_input_maybe(on_input.as_ref().and(Some(InternalMessage::Input)));
        self.on_input = on_input.map(|f| Box::new(f) as _);
        self
    }

    /// Sets the message that should be produced when the [ParsedInput] is
    /// focused and the enter key is pressed.
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self.text_input = self.text_input.on_submit(InternalMessage::Submit);
        self
    }

    /// Sets the message that should be produced when the [ParsedInput] is
    /// focused and the enter key is pressed, if `Some`.
    pub fn on_submit_maybe(mut self, on_submit: Option<Message>) -> Self {
        self.text_input = self.text_input.on_submit_maybe(on_submit.as_ref().and(Some(InternalMessage::Submit)));
        self.on_submit = on_submit;
        self
    }

    /// Sets the message that should be produced when some text is pasted into
    /// the [ParsedInput].
    pub fn on_paste(mut self, on_paste: impl Fn(T) -> Message + 'a) -> Self {
        self.on_paste = Some(Box::new(on_paste));
        self.text_input = self.text_input.on_paste(InternalMessage::Paste);
        self
    }

    /// Sets the message that should be produced when some text is pasted into
    /// the [ParsedInput], if `Some`.
    pub fn on_paste_maybe(
        mut self,
        on_paste: Option<impl Fn(T) -> Message + 'a>,
    ) -> Self {
        self.text_input = self.text_input.on_input_maybe(on_paste.as_ref().and(Some(InternalMessage::Paste)));
        self.on_paste = on_paste.map(|f| Box::new(f) as _);
        self
    }

    /// Sets the [`Font`] of the [ParsedInput].
    ///
    /// [`Font`]: text::Renderer::Font
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.text_input = self.text_input.font(font);
        self
    }

    /// Sets the [`Icon`] of the [ParsedInput].
    pub fn icon(mut self, icon: Icon<Renderer::Font>) -> Self {
        self.text_input = self.text_input.icon(icon);
        self
    }

    /// Sets the width of the [ParsedInput].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.text_input = self.text_input.width(width);
        self
    }

    /// Sets the [`Padding`] of the [ParsedInput].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.text_input = self.text_input.padding(padding);
        self
    }

    /// Sets the text size of the [ParsedInput].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_input = self.text_input.size(size);
        self
    }

    /// Sets the [`text::LineHeight`] of the [ParsedInput].
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_input = self.text_input.line_height(line_height);
        self
    }

    /// Sets the horizontal alignment of the [ParsedInput].
    pub fn align_x(mut self, alignment: impl Into<alignment::Horizontal>) -> Self {
        self.text_input = self.text_input.align_x(alignment);
        self
    }

    /// Sets the style of the [ParsedInput].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.text_input = self.text_input.style(style);
        self
    }

    /// Sets the style class of the [ParsedInput].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.text_input = self.text_input.class(class);
        self
    }
}

impl<'a, T, Message, Theme, Renderer> From<ParsedInput<'a, T, Message, Theme, Renderer>> for Element<'a, Message, Theme, Renderer>
where
    T: 'a,
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(value: ParsedInput<'a, T, Message, Theme, Renderer>) -> Self {
        let elt: Element<'a, InternalMessage, Theme, Renderer> = value.text_input.into(); 
        elt.map(move |msg| match msg {
            InternalMessage::Input(str) => value.on_input.as_ref().unwrap()((value.parser)(str)),
            InternalMessage::Submit => value.on_submit.as_ref().unwrap().clone(),
            InternalMessage::Paste(str) => value.on_paste.as_ref().unwrap()((value.parser)(str))
        })
    }
}

macro_rules! implementor {
    ($trait:tt, $fn:ident, $trait_mut:tt, $fn_mut:ident) => {
        impl<'a, T, Message, Theme, Renderer> $trait<TextInput<'a, InternalMessage, Theme, Renderer>> for ParsedInput<'a, T, Message, Theme, Renderer> 
        where
            Theme: Catalog,
            Renderer: text::Renderer,
        {
            fn $fn(&self) -> &TextInput<'a, InternalMessage, Theme, Renderer> {
                &**self
            }
        }
        
        impl<'a, T, Message, Theme, Renderer> $trait_mut<TextInput<'a, InternalMessage, Theme, Renderer>> for ParsedInput<'a, T, Message, Theme, Renderer> 
        where
            Theme: Catalog,
            Renderer: text::Renderer,
        {
            fn $fn_mut(&mut self) -> &mut TextInput<'a, InternalMessage, Theme, Renderer> {
                &mut **self
            }
        }
    };
}

implementor!{AsRef, as_ref, AsMut, as_mut}
implementor!{Borrow, borrow, BorrowMut, borrow_mut}

impl<'a, T, Message, Theme, Renderer> Deref for ParsedInput<'a, T, Message, Theme, Renderer> 
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    type Target = TextInput<'a, InternalMessage, Theme, Renderer>;

    fn deref(&self) -> &Self::Target {
        &self.text_input
    }
}

impl<'a, T, Message, Theme, Renderer> DerefMut for ParsedInput<'a, T, Message, Theme, Renderer> 
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.text_input
    }
}