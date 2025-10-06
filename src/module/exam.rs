//! A module exam.

use super::*;

/// An exam.
#[derive(Debug, Clone)]
pub struct Exam {
    // Overview
    title: String,
    number: String,

    persons_involved: String,

    typ: String,
    semester_weekly_hours: Duration,
    ects: ECTS,

    offered_in: Semester,
    organisation: String,

    // Description
    further_information: String,

    // Exam information
    assessment_scheme: String,
    note: String,

    // Dates and groups:
    dates: Vec<Appointment>,

    // Registration info
    registration_start: Option<Date>,
    registration_end: Option<Date>,
    deregistration_start: Option<Date>,
    deregistration_end: Option<Date>,
}
// Getters and mutable getters for Exam
impl Exam {
    /// Gets a reference to the exam's further_information field.
    pub fn get_further_information(&self) -> &String {
        &self.further_information
    }
    /// Gets a reference to the exam's title.
    pub fn get_title(&self) -> &String {
        &self.title
    }
    /// Gets a mutable reference to the exam's title.
    pub fn get_mut_title(&mut self) -> &mut String {
        &mut self.title
    }
    /// Gets a reference to the exam's number.
    pub fn get_number(&self) -> &String {
        &self.number
    }
    /// Gets a mutable reference to the exam's number.
    pub fn get_mut_number(&mut self) -> &mut String {
        &mut self.number
    }
    /// Gets a reference to the exam's persons_involved field.
    pub fn get_persons_involved(&self) -> &String {
        &self.persons_involved
    }
    /// Gets a mutable reference to the exam's persons_involved field.
    pub fn get_mut_persons_involved(&mut self) -> &mut String {
        &mut self.persons_involved
    }
    /// Gets a reference to the exam's type.
    pub fn get_typ(&self) -> &String {
        &self.typ
    }
    /// Gets a mutable reference to the exam's type.
    pub fn get_mut_typ(&mut self) -> &mut String {
        &mut self.typ
    }
    /// Gets a reference to the exam's semester_weekly_hours field.
    pub fn get_semester_weekly_hours(&self) -> &Duration {
        &self.semester_weekly_hours
    }
    /// Gets a mutable reference to the exam's semester_weekly_hours field.
    pub fn get_mut_semester_weekly_hours(&mut self) -> &mut Duration {
        &mut self.semester_weekly_hours
    }
    /// Gets a reference to the exam's ECTS value.
    pub fn get_ects(&self) -> &ECTS {
        &self.ects
    }
    /// Gets a mutable reference to the exam's ECTS value.
    pub fn get_mut_ects(&mut self) -> &mut ECTS {
        &mut self.ects
    }
    /// Gets a reference to the exam's offered_in field.
    pub fn get_offered_in(&self) -> &Semester {
        &self.offered_in
    }
    /// Gets a mutable reference to the exam's offered_in field.
    pub fn get_mut_offered_in(&mut self) -> &mut Semester {
        &mut self.offered_in
    }
    /// Gets a reference to the exam's organisation field.
    pub fn get_organisation(&self) -> &String {
        &self.organisation
    }
    /// Gets a mutable reference to the exam's organisation field.
    pub fn get_mut_organisation(&mut self) -> &mut String {
        &mut self.organisation
    }
    /// Gets a reference to the exam's assessment_scheme field.
    pub fn get_assessment_scheme(&self) -> &String {
        &self.assessment_scheme
    }
    /// Gets a mutable reference to the exam's assessment_scheme field.
    pub fn get_mut_assessment_scheme(&mut self) -> &mut String {
        &mut self.assessment_scheme
    }
    /// Gets a reference to the exam's note field.
    pub fn get_note(&self) -> &String {
        &self.note
    }
    /// Gets a mutable reference to the exam's note field.
    pub fn get_mut_note(&mut self) -> &mut String {
        &mut self.note
    }
    /// Gets a reference to the exam's dates.
    pub fn get_dates(&self) -> &Vec<Appointment> {
        &self.dates
    }
    /// Gets a mutable reference to the exam's dates.
    pub fn get_mut_dates(&mut self) -> &mut Vec<Appointment> {
        &mut self.dates
    }
    /// Gets a reference to the exam's registration_start field.
    pub fn get_registration_start(&self) -> &Option<Date> {
        &self.registration_start
    }
    /// Gets a mutable reference to the exam's registration_start field.
    pub fn get_mut_registration_start(&mut self) -> &mut Option<Date> {
        &mut self.registration_start
    }
    /// Gets a reference to the exam's registration_end field.
    pub fn get_registration_end(&self) -> &Option<Date> {
        &self.registration_end
    }
    /// Gets a mutable reference to the exam's registration_end field.
    pub fn get_mut_registration_end(&mut self) -> &mut Option<Date> {
        &mut self.registration_end
    }
    /// Gets a reference to the exam's deregistration_start field.
    pub fn get_deregistration_start(&self) -> &Option<Date> {
        &self.deregistration_start
    }
    /// Gets a mutable reference to the exam's deregistration_start field.
    pub fn get_mut_deregistration_start(&mut self) -> &mut Option<Date> {
        &mut self.deregistration_start
    }
    /// Gets a reference to the exam's deregistration_end field.
    pub fn get_deregistration_end(&self) -> &Option<Date> {
        &self.deregistration_end
    }
    /// Gets a mutable reference to the exam's deregistration_end field.
    pub fn get_mut_deregistration_end(&mut self) -> &mut Option<Date> {
        &mut self.deregistration_end
    }
}
