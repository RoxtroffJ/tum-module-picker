//! Options that represent value or nothing and can be displayed.
//!
//! Some is displayed as the contained value, none is displayed as `-`.

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
    sync::LazyLock,
};

use regex::Regex;

/// The displayable option. Implements [Deref] and [DerefMut].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayOption<T> {
    value: Option<T>,
}

impl<T> Deref for DisplayOption<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for DisplayOption<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> From<Option<T>> for DisplayOption<T> {
    fn from(value: Option<T>) -> Self {
        Self { value }
    }
}

impl<T> From<DisplayOption<T>> for Option<T> {
    fn from(value: DisplayOption<T>) -> Self {
        value.value
    }
}

impl<T: Display> Display for DisplayOption<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &**self {
            Some(x) => write!(f, "{}", x),
            None => write!(f, "-"),
        }
    }
}

impl<T: FromStr> FromStr for DisplayOption<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(v) => Ok(Some(v).into()),
            Err(err) => {
                if is_none(s) {
                    Ok(None.into())
                } else {
                    Err(err)
                }
            }
        }
    }
}

static NONE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\A\s*-\s*\z").unwrap());

fn is_none(s: &str) -> bool {
    s.is_empty() || NONE.is_match(s)
}


/// Takes a normal option and displays it.
pub fn opt_to_string<T: ToString>(value: &Option<T>) -> String {
    match value {
        Some(x) => x.to_string(),
        None => "-".to_string(),
    }
}
