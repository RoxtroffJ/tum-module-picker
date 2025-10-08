use iced::{Element, Task};
use tum_module_picker::{
    module::Module,
    storage_tree::{
        StorageTree as ST,
        column::{Action as STAction, Content as STContent},
    },
};

use crate::module_display::{
    Edit,
    section::{edit_text_editor, section},
};

/// Struct with the content of each different sections.
#[derive(Debug)]
struct SectionContent {
    general: general::Content,
    workload: workload::Content,
    study: study::Content,
    description: description::Content,
}

macro_rules! apply_to_all {
    ($content:expr => $method:ident($($arg:expr),* $(,)?)) => {{
        $content.general.$method($($arg),*);
        $content.workload.$method($($arg),*);
        $content.study.$method($($arg),*);
        $content.description.$method($($arg),*);
    }};
}

impl SectionContent {
    fn set_all_edits(&mut self, value: bool, module: &Module) {
        {
            self.general.set_all_edits(value);
            self.workload.set_all_edits(value, module);
            self.study.set_all_edits(value, module);
            self.description.set_all_edits(value, module);
        }
    }

    fn reset(&mut self, module: &Module) {
        apply_to_all!(self => reset(module))
    }
}

#[derive(Debug)]
enum Sections {
    General,
    WorkLoad,
    StudyPerf,
    Descr,
}

#[derive(Debug)]
enum SectionTitle {
    Description,
    Section(Sections),
}

impl ToString for SectionTitle {
    fn to_string(&self) -> String {
        use SectionTitle::*;
        match self {
            Description => "Description",
            Section(Sections::General) => "General",
            Section(Sections::WorkLoad) => "Workload",
            Section(Sections::StudyPerf) => "Study and examination performance",
            Section(Sections::Descr) => "Description",
        }
        .to_string()
    }
}

pub mod description;
pub mod general;
pub mod study;
pub mod workload;

#[derive(Debug, Clone)]
pub enum Action {
    General(general::Action),
    WorkLoad(workload::Action),
    StudyPerf(study::Action),
    Descr(description::Action),

    StorageTree(STAction),
}

/// Content of the description section.
#[derive(Debug)]
pub struct Content {
    content: STContent<SectionTitle, Sections>,
    section_content: SectionContent,
}

impl Content {
    pub fn new() -> Self {
        macro_rules! singular {
            ($section:ident) => {
                ST::node(
                    SectionTitle::Section(Sections::$section),
                    vec![ST::leaf(Sections::$section)],
                )
            };
        }

        let tree = ST::node(
            SectionTitle::Description,
            vec![
                singular!(General),
                singular!(WorkLoad),
                singular!(StudyPerf),
                singular!(Descr),
            ],
        );

        Self {
            content: STContent::new(tree).retract_on_select(),
            section_content: SectionContent {
                general: general::Content::new(),
                workload: workload::Content::new(),
                study: study::Content::new(),
                description: description::Content::new(),
            },
        }
    }

    pub fn set_all_edits(&mut self, value: bool, module: &Module) {
        self.section_content.set_all_edits(value, module)
    }

    pub fn view<'a>(&'a self, module: &'a Module) -> Element<'a, Action> {
        section(
            SectionTitle::to_string,
            |x| match x {
                Sections::General => self
                    .section_content
                    .general
                    .view(module)
                    .map(Action::General),
                Sections::WorkLoad => self
                    .section_content
                    .workload
                    .view(module)
                    .map(Action::WorkLoad),
                Sections::StudyPerf => self
                    .section_content
                    .study
                    .view(module)
                    .map(Action::StudyPerf),
                Sections::Descr => self
                    .section_content
                    .description
                    .view(module)
                    .map(Action::Descr),
            },
            |x| match x {
                Sections::General => self
                    .section_content
                    .general
                    .get_editable()
                    .as_ref()
                    .and_then(|e| {
                        edit_text_editor(e, &e.editor_content, |a| {
                            Action::General(general::Action::Editor(a))
                        })
                    }),
                Sections::WorkLoad => self
                    .section_content
                    .workload
                    .get_editable()
                    .as_ref()
                    .and_then(|e| {
                        edit_text_editor(e, &e.editor, |a| {
                            Action::WorkLoad(workload::Action::Editor(a))
                        })
                    }),
                Sections::StudyPerf => {
                    self.section_content
                        .study
                        .get_editable()
                        .as_ref()
                        .and_then(|e| {
                            edit_text_editor(e, &e.editor, |a| {
                                Action::StudyPerf(study::Action::Editor(a))
                            })
                        })
                }
                Sections::Descr => self
                    .section_content
                    .description
                    .get_editable()
                    .as_ref()
                    .and_then(|e| {
                        edit_text_editor(e, &e.editor, |a| {
                            Action::Descr(description::Action::Editor(a))
                        })
                    }),
            },
            |x| match x {
                Sections::General => self
                    .section_content
                    .general
                    .get_editable()
                    .as_ref()
                    .map(Edit::has_one_error)
                    .unwrap_or(false),
                Sections::WorkLoad => self
                    .section_content
                    .workload
                    .get_editable()
                    .as_ref()
                    .map(Edit::has_one_error)
                    .unwrap_or(false),
                Sections::StudyPerf => self
                    .section_content
                    .study
                    .get_editable()
                    .as_ref()
                    .map(Edit::has_one_error)
                    .unwrap_or(false),
                Sections::Descr => self
                    .section_content
                    .description
                    .get_editable()
                    .as_ref()
                    .map(Edit::has_one_error)
                    .unwrap_or(false),
            },
            &self.content,
            Action::StorageTree,
        )
        .into()
    }

    pub fn perform(&mut self, action: Action, module: &mut Module) -> Task<Action> {
        match action {
            Action::General(action) => {
                return self
                    .section_content
                    .general
                    .perform(action, module)
                    .map(Action::General);
            }
            Action::WorkLoad(action) => {
                return self
                    .section_content
                    .workload
                    .perform(action, module)
                    .map(Action::WorkLoad);
            }
            Action::StudyPerf(action) => {
                return self
                    .section_content
                    .study
                    .perform(action, module)
                    .map(Action::StudyPerf);
            }
            Action::Descr(action) => {
                return self
                    .section_content
                    .description
                    .perform(action, module)
                    .map(Action::Descr);
            }
            Action::StorageTree(action) => self.content.perform(action),
        };

        Task::none()
    }

    /// Resets the fields to match those of the given [Module].
    pub fn reset(&mut self, module: &Module) {
        self.section_content.reset(module);
    }

    /// Expands or collapses all the collapsable pannels.
    pub fn expand(&mut self, value: bool) {
        self.content.expand_all(value);
    }
}
