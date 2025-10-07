//! Stores data on leafs of a tree.
//!
//! Each tree node has a key to make navigating possible.
//! Also provides [iced] helpers to turn a tree in an [Element](iced::Element).

/// A storage tree with nodes of type `K` (Key) and leafs of type `T`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageTree<K, T> {
    /// A leaf.
    Leaf(T),
    /// A tree node.
    Node(Node<K, T>),
}

/// A tree node that has a key and children [StorageTree]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<K, T> {
    key: K,
    children: Vec<StorageTree<K, T>>,
}

/// A path in a tree.
pub type Path = Vec<usize>;

impl<K, T> StorageTree<K, T> {
    /// Creates a new [StorageTree] leaf.
    pub fn leaf(value: T) -> Self {
        Self::Leaf(value)
    }

    /// Creates a new [StorageTree] node.
    pub fn node(key: K, children: Vec<Self>) -> Self {
        Self::Node(Node { key, children })
    }

    /// Finds the sub tree at the given [Path] if it exists.
    pub fn get(&self, path: &Path) -> Option<&Self> {
        let mut current = self;
        for index in path {
            match current {
                StorageTree::Leaf(_) => return None,
                StorageTree::Node(node) => current = node.get_children().get(*index)?,
            }
        }
        Some(current)
    }

    /// Same as [get](Self::get) but returns a mutable reference.
    pub fn get_mut(&mut self, path: &Path) -> Option<&mut Self> {
        let mut current = self;
        for index in path {
            match current {
                StorageTree::Leaf(_) => return None,
                StorageTree::Node(node) => current = node.get_mut_children().get_mut(*index)?,
            }
        }
        Some(current)
    }

    /// Adds a sub tree at the node pointed by path.
    ///
    /// If this node does not exist, nothing is done.
    /// If successful, returns a mutable reference to the inserted tree.
    pub fn add(&mut self, other: Self, path: &Path) -> Option<&mut Self> {
        self.get_mut(path).and_then(|tree| match tree {
            StorageTree::Leaf(_) => return None,
            StorageTree::Node(node) => {
                let children = node.get_mut_children();
                children.push(other);
                children.last_mut()
            }
        })
    }

    /// Applies a function to all the keys of the [StorageTree].
    ///
    /// The function is called once per key, and in an arbitrary order.
    pub fn map_keys<L, F: Fn(K) -> L>(self, f: &F) -> StorageTree<L, T> {
        match self {
            StorageTree::Leaf(value) => StorageTree::leaf(value),
            StorageTree::Node(node) => StorageTree::node(
                f(node.key),
                node.children
                    .into_iter()
                    .map(|tree| tree.map_keys(f))
                    .collect(),
            ),
        }
    }

    /// Iterator on the leafs of a [StorageTree].
    pub fn leaf_iter(&self) -> LeafIterator<'_, K, T> {
        LeafIterator { stack: vec![self] }
    }

    /// Same as [leaf_iter](Self::leaf_iter) but returns mutable references.
    pub fn mut_leaf_iter(&mut self) -> MutLeafIterator<'_, K, T> {
        MutLeafIterator { stack: vec![self] }
    }
}

impl<K, T> Node<K, T> {
    /// Returns a reference to the key of a node.
    pub fn get_key(&self) -> &K {
        &self.key
    }

    /// Returns a reference to the children of a node.
    pub fn get_children(&self) -> &Vec<StorageTree<K, T>> {
        &self.children
    }

    /// Returns a mutable reference to the key of a node.
    pub fn get_mut_key(&mut self) -> &mut K {
        &mut self.key
    }

    /// Returns a mutable reference to the children of a node.
    pub fn get_mut_children(&mut self) -> &mut Vec<StorageTree<K, T>> {
        &mut self.children
    }
}

/// Iterator on leafs of a tree.
pub struct LeafIterator<'a, K, T> {
    stack: Vec<&'a StorageTree<K, T>>,
}

impl<'a, K, T> Iterator for LeafIterator<'a, K, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().and_then(|tree| match tree {
            StorageTree::Leaf(leaf) => Some(leaf),
            StorageTree::Node(node) => {
                self.stack.extend(node.get_children());
                self.next()
            }
        })
    }
}

/// Same as [LeafIterator] but yields mutable references.
pub struct MutLeafIterator<'a, K, T> {
    stack: Vec<&'a mut StorageTree<K, T>>,
}

impl<'a, K, T> Iterator for MutLeafIterator<'a, K, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().and_then(|tree| match tree {
            StorageTree::Leaf(leaf) => Some(leaf),
            StorageTree::Node(node) => {
                self.stack.extend(node.get_mut_children());
                self.next()
            }
        })
    }
}

pub mod column;
