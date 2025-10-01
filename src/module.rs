//! Contains everything related to the [Module] struct.

use std::{
    collections::HashMap,
    io::BufRead,
    sync::LazyLock,
};

use regex::Regex;
use time::{Date, Duration, Month, PrimitiveDateTime, Time};

pub mod module;

pub mod course;

pub mod exam;

mod appointment;
pub use appointment::*;

mod semester;
pub use semester::*;

/// The type for a year.
pub type Year = u16;
/// The type for an amount of ECTS
pub type ECTS = u16;
