// ==== Imports ====

use super::*;
use iced::Task;
use std::{
    borrow::{Borrow, BorrowMut}, mem, ops::{Deref, DerefMut}
};
use tum_module_picker::module::Module;

mod action;
pub use action::*;

mod perform;

// ==== Content ====

/// State of a [ModuleDisplay].
///
/// It can be [Deref]ed in a [Module].
#[derive(Debug)]
pub struct Content {
    pub module: Module,
    pub overview_content: overview::Content,
    pub description_content: description::Content,
    pub editable: Option<Editable>,
}
impl Content {
    /// Creates a new [Content] from a module.
    pub fn new(module: Module) -> Self {
        Self {
            module,
            overview_content: overview::Content::new(),
            description_content: description::Content::new(),
            editable: None,
        }
    }

    /// Same as self but with all the edits to a value.
    pub fn with_all_edits(mut self, value: bool) -> Self {
        self.set_all_edits(value);
        self
    }

    /// Sets all the edits to a value.
    pub fn set_all_edits(&mut self, value: bool) {
        
        self.overview_content.set_all_edits(value, &self.module);
        self.description_content.set_all_edits(value, &self.module);

        if self.editable.as_ref().is_none() && !value {
            return;
        }

        self.editable.get_or_insert(Editable::new()).set_all(value);
    }

    /// Takes the content and returns the module.
    pub fn take_module(self) -> Module {
        self.module
    }

    /// Replaces the module by given one.
    pub fn replace_module(&mut self, module: Module) -> Module {
        self.overview_content.reset(&module);
        self.description_content.reset(&module);
        let module = mem::replace(&mut self.module, module);
        module
    }

    /// Expands or collapses all the expandable fields.
    pub fn expand_all(&mut self, value: bool) {
        self.overview_content.expand_all(value);
        self.description_content.expand(value);
    }
}

impl AsRef<Module> for Content {
    fn as_ref(&self) -> &Module {
        &**self
    }
}
impl AsMut<Module> for Content {
    fn as_mut(&mut self) -> &mut Module {
        &mut **self
    }
}
impl Borrow<Module> for Content {
    fn borrow(&self) -> &Module {
        &**self
    }
}
impl BorrowMut<Module> for Content {
    fn borrow_mut(&mut self) -> &mut Module {
        &mut **self
    }
}

impl Deref for Content {
    type Target = Module;

    fn deref(&self) -> &Self::Target {
        &self.module
    }
}

impl DerefMut for Content {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.module
    }
}
