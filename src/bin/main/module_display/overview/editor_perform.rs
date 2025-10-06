use super::Editable;
use std::ops::Deref;

use iced::{Task, advanced::graphics::futures::MaybeSend, widget::text_editor};
use tum_module_picker::{
    borrow_map::BorrowMap,
    module::semester::Semester,
    sectioned_lines::get_sections,
};

use crate::module_display::overview::Action;

#[derive(Debug)]
enum SideSections {
    Name,
    Id,
    ECTS,
    Version,
    Valid,
    Responsible,
    Organisation,
    Note,
}

impl SideSections {
    fn to_action(&self) -> impl Fn(String) -> Action {
        match self {
            SideSections::ECTS => Action::ECTS,
            SideSections::Version => Action::Version,
            SideSections::Responsible => Action::Responsible,
            SideSections::Organisation => Action::Organisation,
            SideSections::Note => Action::Note,
            _ => panic!("No overview::action found for {self:?}"),
        }
    }
}

pub(super) fn perform<Message: MaybeSend + 'static>(
    action: text_editor::Action,
    editable: Option<&mut Editable>,

    on_action: &impl Fn(Action) -> Message,
    name_edit: &impl Fn(String) -> Message,
    id_edit: &impl Fn(String) -> Message,
) -> Task<Message> {
    if let Some(editable) = editable {
        editable.overview_content.perform(action);

        let mut tasks = Vec::new();

        let mut section_iter = get_sections(editable.overview_content.lines(), &|str| {
            use SideSections::*;
            match str {
                "Name" => Some(Name),
                "Module ID" => Some(Id),
                "ECTS credits" => Some(ECTS),
                "Version" => Some(Version),
                "Valid" => Some(Valid),
                "Responsible for Module" => Some(Responsible),
                "Organisation" => Some(Organisation),
                "Note" => Some(Note),
                _ => None,
            }
        });

        while let Some(field) = section_iter.next_section() {
            match field {
                SideSections::Valid => {
                    let str = section_iter
                        .borrow_map(|x| {
                            let mut res = x.deref().to_string();
                            res.push('\n');
                            res
                        })
                        .fold(String::new(), |mut a, b| {
                            a.push_str(&b);
                            a
                        });

                    match Semester::parse_with_index(&str) {
                        Ok((from_semester, _start, stop)) => {
                            tasks.push(Task::done(on_action(Action::ValidFromParsed(Some(
                                from_semester,
                            )))));
                            match Semester::parse_with_index(&str[stop..]) {
                                Ok((until_semester, _, _)) => tasks.push(Task::done(on_action(
                                    Action::ValidUntilParsed(Some(until_semester)),
                                ))),
                                Err(_) => tasks
                                    .push(Task::done(on_action(Action::ValidUntilParsed(None)))),
                            }
                        }
                        Err(_) => tasks.push(Task::done(on_action(Action::ValidFromParsed(None)))),
                    }
                }
                SideSections::Name => set_and_send(&mut section_iter, &mut tasks, name_edit),
                SideSections::Id => set_and_send(&mut section_iter, &mut tasks, id_edit),

                _ => set_and_send(&mut section_iter, &mut tasks, |x| {
                    on_action(field.to_action()(x))
                }),
            }
        }

        return Task::batch(tasks);
    }
    Task::none()
}

fn set_and_send<Message: MaybeSend + 'static>(
    section_iter: &mut impl Iterator<Item = impl Deref<Target = str>>,
    tasks: &mut Vec<Task<Message>>,

    message: impl Fn(String) -> Message,
) {
    tasks.push(Task::done(message(
        section_iter
            .next()
            .as_deref()
            .unwrap_or_default()
            .to_string(),
    )))
}
