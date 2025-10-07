//! [iced] component to display a [StorageTree] in a column.

use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

use iced::{
    Element,
    Length::{self, Shrink},
    Pixels,
    alignment::Vertical,
    widget::{Space, button, column, horizontal_space, row},
};
use iced_fonts::required::{RequiredIcons, to_text};

use super::*;

/// [iced] component for a [StorageTree]
pub struct Column<'a, K, T, Message> {
    content: &'a Content<K, T>,
    icons: Option<Icons<'a, Message>>,
    space: f32,
    on_interaction: Box<dyn Fn(Action) -> Message + 'a>,
    key_to_element:
        Box<dyn Fn(&'a Node<MetaKey<K, NodeState>, T>, Path) -> Element<'a, Message> + 'a>,
    value_to_element: Box<dyn Fn(&'a T, Path) -> Element<'a, Message> + 'a>,

    spacing: Pixels,
}

/// Content and state of a [Column]
#[derive(Debug)]
pub struct Content<K, T> {
    st: StorageTree<MetaKey<K, NodeState>, T>,
    just_extend_on_select: bool,
}

impl<K, T> Content<K, T> {
    /// Creates a new [Content]
    pub fn new(storage_tree: StorageTree<K, T>) -> Self {
        let st = storage_tree.map_keys(&|key| MetaKey::new(key, NodeState::new()));
        Self {
            st,
            just_extend_on_select: true,
        }
    }

    /// Performs an action on the content.
    pub fn perform(&mut self, action: Action) {
        match action {
            Action::Expand(path) => self.set_expanded(&path, true),
            Action::Collapse(path) => self.set_expanded(&path, false),
            Action::Selected(path) => self.set_expanded(
                &path,
                if self.just_extend_on_select {
                    true
                } else {
                    !self.get_expanded(&path).unwrap_or(true)
                },
            ),
        }
    }

    /// Edits the expanded status of a node.
    fn set_expanded(&mut self, path: &Path, expanded: bool) {
        self.get_mut(&path).map(|tree| match tree {
            StorageTree::Leaf(_) => (),
            StorageTree::Node(node) => node.get_mut_key().get_mut_metadata().expanded = expanded,
        });
    }

    /// Returns whether a node is expanded or not.
    fn get_expanded(&self, path: &Path) -> Option<bool> {
        self.get(path).and_then(|tree| match tree {
            StorageTree::Leaf(_) => None,
            StorageTree::Node(node) => Some(node.get_key().get_metadata().expanded),
        })
    }

    /// Makes it so that nodes both expand and collapse when clicked.
    ///
    /// If this function is not called, they just expand when clicked.
    pub fn retract_on_select(self) -> Self {
        Self {
            just_extend_on_select: false,
            ..self
        }
    }
}

struct Icons<'a, Message> {
    collapsed: Box<dyn Fn() -> Element<'a, Message> + 'a>,
    expanded: Box<dyn Fn() -> Element<'a, Message> + 'a>,
}

/// GUI state of a node.
#[derive(Debug)]
pub struct NodeState {
    expanded: bool,
}

/// Key with some metadata attached to it.
#[derive(Debug)]
pub struct MetaKey<K, M> {
    key: K,
    metadata: M,
}

/// An interaction with a [Column].
///
/// Upon recieving one, they must be forwared to the [perform](Content::perform) function.
#[derive(Debug, Clone)]
pub enum Action {
    /// Expands a node in the tree.
    Expand(Path),
    /// Collapses a node in the tree.
    Collapse(Path),
    /// Indicates that the node at given path was selected.
    ///
    /// Feel free to intercept this message and do something before forwarding it to [perform](Content::perform)
    Selected(Path),
}

impl<'a, K, T, Message> Column<'a, K, T, Message> {
    /// Creates a new [Column] when given a [StorageTree]
    pub fn new(
        content: &'a Content<K, T>,
        on_interaction: impl Fn(Action) -> Message + 'a,
        key_to_element: impl Fn(&'a Node<MetaKey<K, NodeState>, T>, Path) -> Element<'a, Message> + 'a,
        value_to_element: impl Fn(&'a T, Path) -> Element<'a, Message> + 'a,
    ) -> Self {
        Self {
            content,
            icons: None,
            space: 0.,
            on_interaction: Box::new(on_interaction),
            key_to_element: Box::new(key_to_element),
            value_to_element: Box::new(value_to_element),
            spacing: 0.into(),
        }
    }

    /// Sets the icons when collapsed and expanded.
    ///
    /// If this method is not called, there will be no icons.
    pub fn icon<C: Fn() -> Element<'a, Message> + 'a, E: Fn() -> Element<'a, Message> + 'a>(
        self,
        collapsed: C,
        expanded: E,
    ) -> Self {
        Self {
            icons: Some(Icons::new(collapsed, expanded)),
            ..self
        }
    }

    /// Same as [icon](Self::icon) if [Some], and removes the icons if [None].
    pub fn icon_maybe<
        C: Fn() -> Element<'a, Message> + 'a,
        E: Fn() -> Element<'a, Message> + 'a,
    >(
        self,
        icons: Option<(C, E)>,
    ) -> Self {
        match icons {
            None => self,
            Some((collapsed, expanded)) => self.icon(collapsed, expanded),
        }
    }

    /// Puts default icons for collapsed and expanded.
    pub fn icons_default<P: Into<Length> + Copy + 'a>(self, padding: P) -> Self
    where
        Message: 'a,
    {
        self.icon(
            move || {
                let result: Element<'a, Message> = row![
                    to_text(RequiredIcons::CaretRightFill),
                    horizontal_space().width(padding)
                ]
                .into();
                result
            },
            move || {
                row![
                    to_text(RequiredIcons::CaretDownFill),
                    horizontal_space().width(padding)
                ]
                .into()
            },
        )
    }

    /// Sets by how much the children of a node are offset to the right compared to said node.
    pub fn space(self, space: f32) -> Self {
        Self { space, ..self }
    }

    /// Sets the spacing of the inner column.
    pub fn spacing(self, spacing: impl Into<Pixels>) -> Self {
        Self {
            spacing: spacing.into(),
            ..self
        }
    }
}

impl<K, T> StorageTree<MetaKey<K, NodeState>, T> {
    /// Draws the content
    fn to_element<'a, Message: Clone + 'a, Oi, Ok, Ov>(
        &'a self,
        icons: &Option<Icons<'a, Message>>,
        space: f32,
        spacing: Pixels,
        on_interaction: &Oi,
        key_to_element: &Ok,
        value_to_element: &Ov,
        current_path: Path,
    ) -> Element<'a, Message>
    where
        Oi: Fn(Action) -> Message,
        Ok: Fn(&'a Node<MetaKey<K, NodeState>, T>, Path) -> Element<'a, Message>,
        Ov: Fn(&'a T, Path) -> Element<'a, Message>,
    {
        match &self {
            StorageTree::Leaf(content) => value_to_element(content, current_path),
            StorageTree::Node(node) => {
                let metakey = node.get_key();
                let expanded = metakey.get_metadata().expanded;

                let key_element: Element<'_, _> =
                    button(key_to_element(node, current_path.clone()))
                        .on_press(on_interaction(Action::Selected(current_path.clone())))
                        .padding(0)
                        .style(button::text)
                        .into();

                let row = match icons {
                    Some(icons) => {
                        let icon = if expanded {
                            button((icons.expanded)())
                                .on_press(on_interaction(Action::Collapse(current_path.clone())))
                        } else {
                            button((icons.collapsed)())
                                .on_press(on_interaction(Action::Expand(current_path.clone())))
                        }
                        .style(button::text)
                        .padding(0);
                        row![icon, key_element].align_y(Vertical::Center).into()
                    }
                    None => key_element,
                };

                if expanded {
                    let expansion = node.get_children().iter().enumerate().map(|(idx, tree)| {
                        let mut new_path = current_path.clone();
                        new_path.push(idx);
                        tree.to_element(
                            icons,
                            space,
                            spacing,
                            on_interaction,
                            key_to_element,
                            value_to_element,
                            new_path,
                        )
                    });
                    column![
                        row,
                        row![
                            horizontal_space().width(Length::Fixed(space)),
                            iced::widget::Column::from_iter(expansion).spacing(spacing)
                        ],
                    ]
                    .spacing(spacing)
                    .into()
                } else {
                    row
                }
            }
        }
    }
}

impl<'a, K, T, Message: Clone + 'a> From<Column<'a, K, T, Message>> for Element<'a, Message> {
    fn from(value: Column<'a, K, T, Message>) -> Self {
        value.content.to_element(
            &value.icons,
            value.space,
            value.spacing,
            &value.on_interaction,
            &value.key_to_element,
            &value.value_to_element,
            Vec::new(),
        )
    }
}

impl<K, M> MetaKey<K, M> {
    /// Creates a new [MetaKey].
    pub fn new(key: K, metadata: M) -> Self {
        Self { key, metadata }
    }

    /// Returns a reference to the metadata.
    pub fn get_metadata(&self) -> &M {
        &self.metadata
    }

    /// Returns a mutable reference to the metadata.
    pub fn get_mut_metadata(&mut self) -> &mut M {
        &mut self.metadata
    }

    /// Takes the [MetaKey] and returns both the key and metadata.
    pub fn take(self) -> (K, M) {
        (self.key, self.metadata)
    }
}

impl NodeState {
    /// Creates a new [NodeState].
    fn new() -> Self {
        Self { expanded: false }
    }
}

impl<'a, Message> Icons<'a, Message> {
    fn new<C: Fn() -> Element<'a, Message> + 'a, E: Fn() -> Element<'a, Message> + 'a>(
        collapsed: C,
        expanded: E,
    ) -> Self {
        Self {
            collapsed: Box::new(collapsed),
            expanded: Box::new(expanded),
        }
    }
}

impl Default for NodeState {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, T> AsRef<StorageTree<MetaKey<K, NodeState>, T>> for Content<K, T> {
    fn as_ref(&self) -> &StorageTree<MetaKey<K, NodeState>, T> {
        &**self
    }
}

impl<K, T> AsMut<StorageTree<MetaKey<K, NodeState>, T>> for Content<K, T> {
    fn as_mut(&mut self) -> &mut StorageTree<MetaKey<K, NodeState>, T> {
        &mut **self
    }
}

impl<K, T> Borrow<StorageTree<MetaKey<K, NodeState>, T>> for Content<K, T> {
    fn borrow(&self) -> &StorageTree<MetaKey<K, NodeState>, T> {
        &**self
    }
}

impl<K, T> BorrowMut<StorageTree<MetaKey<K, NodeState>, T>> for Content<K, T> {
    fn borrow_mut(&mut self) -> &mut StorageTree<MetaKey<K, NodeState>, T> {
        &mut **self
    }
}

impl<K, T> Deref for Content<K, T> {
    type Target = StorageTree<MetaKey<K, NodeState>, T>;

    fn deref(&self) -> &Self::Target {
        &self.st
    }
}

impl<K, T> DerefMut for Content<K, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.st
    }
}

impl<K, M> AsRef<K> for MetaKey<K, M> {
    fn as_ref(&self) -> &K {
        &**self
    }
}

impl<K, M> AsMut<K> for MetaKey<K, M> {
    fn as_mut(&mut self) -> &mut K {
        &mut **self
    }
}

impl<K, M> Borrow<K> for MetaKey<K, M> {
    fn borrow(&self) -> &K {
        &**self
    }
}

impl<K, M> BorrowMut<K> for MetaKey<K, M> {
    fn borrow_mut(&mut self) -> &mut K {
        &mut **self
    }
}

impl<K, M> Deref for MetaKey<K, M> {
    type Target = K;

    fn deref(&self) -> &Self::Target {
        &self.key
    }
}

impl<K, M> DerefMut for MetaKey<K, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.key
    }
}

impl<K, M: Default> From<K> for MetaKey<K, M> {
    fn from(value: K) -> Self {
        Self::new(value, M::default())
    }
}
