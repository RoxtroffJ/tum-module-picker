use super::*;

/// A semester. For example, 2025 summer semester.
#[derive(Debug, Clone)]
pub struct Semester {
    year: Year,
    winter: bool,
}
// Getters and mutable getters for Semester
impl Semester {
    /// Gets a reference to the semester's year.
    pub fn get_year(&self) -> &Year {
        &self.year
    }
    /// Gets a mutable reference to the semester's year.
    pub fn get_mut_year(&mut self) -> &mut Year {
        &mut self.year
    }
    /// Gets a reference to the semester's winter field.
    pub fn get_winter(&self) -> &bool {
        &self.winter
    }
    /// Gets a mutable reference to the semester's winter field.
    pub fn get_mut_winter(&mut self) -> &mut bool {
        &mut self.winter
    }
}
