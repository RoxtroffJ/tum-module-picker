
use super::*;

/// Actions that [Content] can [perform](Content::perform).
#[derive(Debug, Clone)]
pub enum Action {
    Name(String),
    Id(String),

    Overview(overview::Action)
}
