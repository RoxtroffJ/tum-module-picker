//! Contains everything related to the [Module] struct.

/// A module.
#[derive(Debug, Clone)]
pub struct Module {
    id: String,
    title: String,
    courses: Vec<Course>,
    exams: Vec<Exam>
}

/// A course.
#[derive(Debug, Clone)]
pub struct Course {
    title: String,
    number: String,
    typ: String
}

/// An exam.
#[derive(Debug, Clone)]
pub struct Exam {
    title: String,
    number: String,
    typ: String
}