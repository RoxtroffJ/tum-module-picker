//! A course in a module.


use super::*;

/// A course.
#[derive(Debug, Clone)]
pub struct Course {
    // Overview
    title: String,
    number: String,

    head: String,
    lecturers: String,

    typ: String,
    semester_weekly_hours: Duration,
    ects: ECTS,

    language: String,
    offered_in: Semester,
    organisation: String,

    // Description
    content: String,
    previous_knowledge_expected: String,
    objective: String,
    teaching_and_learning_method: String,
    course_criteria_and_registration: String,

    recomended_reading: String,
    note: String,

    // Dates and groups:
    dates: HashMap<String, Vec<Appointment>>,

    // Registration info:
    registration_start: Option<Date>,
    registration_end: Option<Date>,
    deregistration_start: Option<Date>,
    deregistration_end: Option<Date>,
}
// Getters and mutable getters for Course
impl Course {
    /// Gets a reference to the course's course_criteria_and_registration field.
    pub fn get_course_criteria_and_registration(&self) -> &String {
        &self.course_criteria_and_registration
    }
    /// Gets a mutable reference to the course's course_criteria_and_registration field.
    pub fn get_mut_course_criteria_and_registration(&mut self) -> &mut String {
        &mut self.course_criteria_and_registration
    }
    /// Gets a mutable reference to the course's teaching_and_learning_method field.
    pub fn get_mut_teaching_and_learning_method(&mut self) -> &mut String {
        &mut self.teaching_and_learning_method
    }
    /// Gets a reference to the course's recomended_reading field.
    pub fn get_recomended_reading(&self) -> &String {
        &self.recomended_reading
    }
    /// Gets a reference to the course's title.
    pub fn get_title(&self) -> &String {
        &self.title
    }
    /// Gets a mutable reference to the course's title.
    pub fn get_mut_title(&mut self) -> &mut String {
        &mut self.title
    }
    /// Gets a reference to the course's number.
    pub fn get_number(&self) -> &String {
        &self.number
    }
    /// Gets a mutable reference to the course's number.
    pub fn get_mut_number(&mut self) -> &mut String {
        &mut self.number
    }
    /// Gets a reference to the course's head.
    pub fn get_head(&self) -> &String {
        &self.head
    }
    /// Gets a mutable reference to the course's head.
    pub fn get_mut_head(&mut self) -> &mut String {
        &mut self.head
    }
    /// Gets a reference to the course's lecturers.
    pub fn get_lecturers(&self) -> &String {
        &self.lecturers
    }
    /// Gets a mutable reference to the course's lecturers.
    pub fn get_mut_lecturers(&mut self) -> &mut String {
        &mut self.lecturers
    }
    /// Gets a reference to the course's type.
    pub fn get_typ(&self) -> &String {
        &self.typ
    }
    /// Gets a mutable reference to the course's type.
    pub fn get_mut_typ(&mut self) -> &mut String {
        &mut self.typ
    }
    /// Gets a reference to the course's semester_weekly_hours field.
    pub fn get_semester_weekly_hours(&self) -> &Duration {
        &self.semester_weekly_hours
    }
    /// Gets a mutable reference to the course's semester_weekly_hours field.
    pub fn get_mut_semester_weekly_hours(&mut self) -> &mut Duration {
        &mut self.semester_weekly_hours
    }
    /// Gets a reference to the course's ECTS value.
    pub fn get_ects(&self) -> &ECTS {
        &self.ects
    }
    /// Gets a mutable reference to the course's ECTS value.
    pub fn get_mut_ects(&mut self) -> &mut ECTS {
        &mut self.ects
    }
    /// Gets a reference to the course's language field.
    pub fn get_language(&self) -> &String {
        &self.language
    }
    /// Gets a mutable reference to the course's language field.
    pub fn get_mut_language(&mut self) -> &mut String {
        &mut self.language
    }
    /// Gets a reference to the course's offered_in field.
    pub fn get_offered_in(&self) -> &Semester {
        &self.offered_in
    }
    /// Gets a mutable reference to the course's offered_in field.
    pub fn get_mut_offered_in(&mut self) -> &mut Semester {
        &mut self.offered_in
    }
    /// Gets a reference to the course's organisation field.
    pub fn get_organisation(&self) -> &String {
        &self.organisation
    }
    /// Gets a mutable reference to the course's organisation field.
    pub fn get_mut_organisation(&mut self) -> &mut String {
        &mut self.organisation
    }
    /// Gets a reference to the course's content field.
    pub fn get_content(&self) -> &String {
        &self.content
    }
    /// Gets a mutable reference to the course's content field.
    pub fn get_mut_content(&mut self) -> &mut String {
        &mut self.content
    }
    /// Gets a reference to the course's previous_knowledge_expected field.
    pub fn get_previous_knowledge_expected(&self) -> &String {
        &self.previous_knowledge_expected
    }
    /// Gets a mutable reference to the course's previous_knowledge_expected field.
    pub fn get_mut_previous_knowledge_expected(&mut self) -> &mut String {
        &mut self.previous_knowledge_expected
    }
    /// Gets a reference to the course's objective field.
    pub fn get_objective(&self) -> &String {
        &self.objective
    }
    /// Gets a mutable reference to the course's objective field.
    pub fn get_mut_objective(&mut self) -> &mut String {
        &mut self.objective
    }
    /// Gets a reference to the course's teaching_and_learning_method field.
    pub fn get_teaching_and_learning_method(&self) -> &String {
        &self.teaching_and_learning_method
    }
    /// Gets a mutable reference to the course's teaching_and_learning_method field.
    /// Gets a mutable reference to the course's recomended_reading field.
    pub fn get_mut_recomended_reading(&mut self) -> &mut String {
        &mut self.recomended_reading
    }
    /// Gets a reference to the course's note field.
    pub fn get_note(&self) -> &String {
        &self.note
    }
    /// Gets a mutable reference to the course's note field.
    pub fn get_mut_note(&mut self) -> &mut String {
        &mut self.note
    }
    /// Gets a reference to the course's dates.
    pub fn get_dates(&self) -> &HashMap<String, Vec<Appointment>> {
        &self.dates
    }
    /// Gets a mutable reference to the course's dates.
    pub fn get_mut_dates(&mut self) -> &mut HashMap<String, Vec<Appointment>> {
        &mut self.dates
    }
    /// Gets a reference to the course's registration_start field.
    pub fn get_registration_start(&self) -> &Option<Date> {
        &self.registration_start
    }
    /// Gets a mutable reference to the course's registration_start field.
    pub fn get_mut_registration_start(&mut self) -> &mut Option<Date> {
        &mut self.registration_start
    }
    /// Gets a reference to the course's registration_end field.
    pub fn get_registration_end(&self) -> &Option<Date> {
        &self.registration_end
    }
    /// Gets a mutable reference to the course's registration_end field.
    pub fn get_mut_registration_end(&mut self) -> &mut Option<Date> {
        &mut self.registration_end
    }
    /// Gets a reference to the course's deregistration_start field.
    pub fn get_deregistration_start(&self) -> &Option<Date> {
        &self.deregistration_start
    }
    /// Gets a mutable reference to the course's deregistration_start field.
    pub fn get_mut_deregistration_start(&mut self) -> &mut Option<Date> {
        &mut self.deregistration_start
    }
    /// Gets a reference to the course's deregistration_end field.
    pub fn get_deregistration_end(&self) -> &Option<Date> {
        &self.deregistration_end
    }
    /// Gets a mutable reference to the course's deregistration_end field.
    pub fn get_mut_deregistration_end(&mut self) -> &mut Option<Date> {
        &mut self.deregistration_end
    }
}
