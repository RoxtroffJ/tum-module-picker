use super::*;

impl Content {
    /// Performs an action
    pub fn perform(&mut self, action: Action) -> Task<Action> {
        match action {
            Action::Name(str) => set_str_field!(self, &self.editable, str, name),
            Action::Id(str) => set_str_field!(self, &self.editable, str, id),
            Action::Overview(action) => {
                return self.overview_content.perform(
                    action,
                    &mut self.module,
                    Action::Overview,
                    Action::Name,
                    Action::Id,
                );
            }
            Action::Description(action) => return self.description_content.perform(action, &mut self.module).map(Action::Description),
        }

        Task::none()
    }
}
