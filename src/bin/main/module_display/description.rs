use iced::{Element, Task};
use iced_aw::Spinner;
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

/// Enum representing each different sections.
#[derive(Debug)]
enum SectionContent {
    General(general::Content),
    WorkLoad(()),
    StudyPerf(()),
    Descr(()),
    Resp(()),
}

#[derive(Debug)]
enum Sections {
    Description,

    General,
    WorkLoad,
    StudyPerf,
    Descr,
    Resp,
}

impl ToString for Sections {
    fn to_string(&self) -> String {
        match self {
            Sections::Description => "Description",
            Sections::General => "General",
            Sections::WorkLoad => "Workload",
            Sections::StudyPerf => "Study and examination performance",
            Sections::Descr => "Description",
            Sections::Resp => "Responsible for module",
        }
        .to_string()
    }
}

pub mod general;

#[derive(Debug, Clone)]
pub enum Action {
    General(general::Action),
    WorkLoad(()),
    StudyPerf(()),
    Descr(()),
    Resp(()),

    StorageTree(STAction),
}

/// Content of the description section.
#[derive(Debug)]
pub struct Content {
    content: STContent<Sections, SectionContent>,
}

impl Content {
    pub fn new() -> Self {
        macro_rules! singular {
            ($section:ident, $new:expr) => {
                ST::node(
                    Sections::$section,
                    vec![ST::leaf(SectionContent::$section($new))],
                )
            };
        }

        let tree = ST::node(
            Sections::Description,
            vec![
                singular!(General, general::Content::new()),
                singular!(WorkLoad, ()),
                singular!(StudyPerf, ()),
                singular!(Descr, ()),
                singular!(Resp, ()),
            ],
        );

        Self {
            content: STContent::new(tree).retract_on_select(),
        }
    }

    pub fn set_all_edits(&mut self, value: bool, _module: &Module) {
        self.content
            .mut_leaf_iter()
            .map(|x| match x {
                SectionContent::General(content) => {
                    content.set_all_edits(value);
                }
                SectionContent::WorkLoad(_) => (),
                SectionContent::StudyPerf(_) => (),
                SectionContent::Descr(_) => (),
                SectionContent::Resp(_) => (),
            })
            .collect()
    }

    pub fn view<'a>(&'a self, module: &'a Module) -> Element<'a, Action> {
        section(
            |x| Sections::to_string(x),
            |x| match x {
                SectionContent::General(content) => content.view(module).map(Action::General),
                SectionContent::WorkLoad(_) => Spinner::new().into(),
                SectionContent::StudyPerf(_) => Spinner::new().into(),
                SectionContent::Descr(_) => Spinner::new().into(),
                SectionContent::Resp(_) => Spinner::new().into(),
            },
            |x| match x {
                SectionContent::General(content) => content.get_editable().as_ref().and_then(|e| {
                    edit_text_editor(e, &e.editor_content, |a| {
                        Action::General(general::Action::Editor(a))
                    })
                }),
                SectionContent::WorkLoad(_) => None,
                SectionContent::StudyPerf(_) => None,
                SectionContent::Descr(_) => None,
                SectionContent::Resp(_) => None,
            },
            |x| match x {
                SectionContent::General(content) => content
                    .get_editable()
                    .as_ref()
                    .map(Edit::has_one_error)
                    .unwrap_or(false),
                SectionContent::WorkLoad(_) => false,
                SectionContent::StudyPerf(_) => false,
                SectionContent::Descr(_) => false,
                SectionContent::Resp(_) => false,
            },
            &self.content,
            Action::StorageTree,
        )
        .into()
    }

    pub fn perform(&mut self, action: Action, module: &mut Module) -> Task<Action> {
        match action {
            Action::General(action) => {
                for sec in self.content.mut_leaf_iter() {
                    match sec {
                        SectionContent::General(content) => {
                            return content.perform(action, module).map(Action::General);
                        }
                        _ => (),
                    }
                }
            }
            Action::WorkLoad(_) => (),
            Action::StudyPerf(_) => (),
            Action::Descr(_) => (),
            Action::Resp(_) => (),
            Action::StorageTree(action) => self.content.perform(action),
        };

        Task::none()
    }
}
