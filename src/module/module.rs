//! [Module] struct and macros that comes with it.
use super::{course::*, exam::*, *};

mod macros;

/// A module.
#[derive(Debug, Clone, Default)]
pub struct Module {
    // === Overview ===
    #[allow(missing_docs)]
    #[allow(missing_docs)]
    pub name: String,
    #[allow(missing_docs)]
    pub id: String,
    #[allow(missing_docs)]
    pub ects: ECTS,
    #[allow(missing_docs)]
    pub version: String,
    #[allow(missing_docs)]
    pub valid_from: Option<Semester>,
    #[allow(missing_docs)]
    pub valid_until: Option<Semester>,
    #[allow(missing_docs)]
    pub responsible: String,
    #[allow(missing_docs)]
    pub organisation: String,
    #[allow(missing_docs)]
    pub note: String,

    // === Courses and exams ===
    #[allow(missing_docs)]
    pub courses: Vec<Course>,
    #[allow(missing_docs)]
    pub exams: Vec<Exam>,

    // === Description ===
    // General data
    #[allow(missing_docs)]
    pub module_level: String,
    #[allow(missing_docs)]
    pub abbreviation: String,
    #[allow(missing_docs)]
    pub subtitle: String,
    #[allow(missing_docs)]
    pub duration: String,
    #[allow(missing_docs)]
    pub occurence: String,
    #[allow(missing_docs)]
    pub language: String,
    #[allow(missing_docs)]
    pub related_programs: String,

    // Work load
    #[allow(missing_docs)]
    pub total_hours: Duration,
    #[allow(missing_docs)]
    pub contact_hours: Duration,
    #[allow(missing_docs)]
    pub self_study_hours: Duration,

    // Study and examination performance
    #[allow(missing_docs)]
    pub descr_of_achievement_assessment_methods: String,
    #[allow(missing_docs)]
    pub exam_retake_next_semester: bool,
    #[allow(missing_docs)]
    pub exam_retake_end_semester: bool,

    // Description
    #[allow(missing_docs)]
    pub prerequisites: String,
    #[allow(missing_docs)]
    pub intended_learning_outcomes: String,
    #[allow(missing_docs)]
    pub content: String,
    #[allow(missing_docs)]
    pub teaching_and_learning_methods: String,
    #[allow(missing_docs)]
    pub media: String,
    #[allow(missing_docs)]
    pub reading_list: String,

    // Responsible for module
    #[allow(missing_docs)]
    pub responsible_bis: String,
}

/// Enum representing the different fields.
#[derive(Debug, Clone, Copy)]
pub enum Field {
    // === Overview ===
    #[allow(missing_docs)]
    Name,
    #[allow(missing_docs)]
    Id,
    #[allow(missing_docs)]
    ECTS,
    #[allow(missing_docs)]
    Version,
    #[allow(missing_docs)]
    ValidFrom,
    #[allow(missing_docs)]
    ValidUntil,
    #[allow(missing_docs)]
    Responsible,
    #[allow(missing_docs)]
    Organisation,
    #[allow(missing_docs)]
    Note,
    // === Courses and exams ===
    #[allow(missing_docs)]
    Courses,
    #[allow(missing_docs)]
    Exams,
    // === Description ===
    // General data
    #[allow(missing_docs)]
    ModuleLevel,
    #[allow(missing_docs)]
    Abbreviation,
    #[allow(missing_docs)]
    Subtitle,
    #[allow(missing_docs)]
    Duration,
    #[allow(missing_docs)]
    Occurence,
    #[allow(missing_docs)]
    Language,
    #[allow(missing_docs)]
    RelatedPrograms,
    // Work load
    #[allow(missing_docs)]
    TotalHours,
    #[allow(missing_docs)]
    ContactHours,
    #[allow(missing_docs)]
    SelfStudyHours,
    // Study and examination performance
    #[allow(missing_docs)]
    DescrOfAchievementAssessmentMethods,
    #[allow(missing_docs)]
    ExamRetakeNextSemester,
    #[allow(missing_docs)]
    ExamRetakeEndSemester,
    // Description
    #[allow(missing_docs)]
    Prerequisites,
    #[allow(missing_docs)]
    IntendedLearningOutcomes,
    #[allow(missing_docs)]
    Content,
    #[allow(missing_docs)]
    TeachingAndLearningMethods,
    #[allow(missing_docs)]
    Media,
    #[allow(missing_docs)]
    ReadingList,
    // Responsible for module
    #[allow(missing_docs)]
    ResponsibleBis,
}

// Getters and mutable getters for Module
impl Module {
    /// Gets a reference to the module's language field.
    pub fn get_language(&self) -> &String {
        &self.language
    }
    /// Gets a mutable reference to the module's language field.
    pub fn get_mut_language(&mut self) -> &mut String {
        &mut self.language
    }
    /// Gets a reference to the module's ects value.
    pub fn get_ects(&self) -> &ECTS {
        &self.ects
    }
    /// Gets a mutable reference to the module's ects value.
    pub fn get_mut_ects(&mut self) -> &mut ECTS {
        &mut self.ects
    }
    /// Gets a reference to the module's version.
    pub fn get_version(&self) -> &String {
        &self.version
    }
    /// Gets a mutable reference to the module's version.
    pub fn get_mut_version(&mut self) -> &mut String {
        &mut self.version
    }
    /// Gets a reference to the module's valid_from field.
    pub fn get_valid_from(&self) -> &Option<Semester> {
        &self.valid_from
    }
    /// Gets a mutable reference to the module's valid_from field.
    pub fn get_mut_valid_from(&mut self) -> &mut Option<Semester> {
        &mut self.valid_from
    }
    /// Gets a reference to the module's valid_until field.
    pub fn get_valid_until(&self) -> &Option<Semester> {
        &self.valid_until
    }
    /// Gets a mutable reference to the module's valid_until field.
    pub fn get_mut_valid_until(&mut self) -> &mut Option<Semester> {
        &mut self.valid_until
    }
    /// Gets a reference to the module's responsible field.
    pub fn get_responsible(&self) -> &String {
        &self.responsible
    }
    /// Gets a mutable reference to the module's responsible field.
    pub fn get_mut_responsible(&mut self) -> &mut String {
        &mut self.responsible
    }
    /// Gets a reference to the module's organisation field.
    pub fn get_organisation(&self) -> &String {
        &self.organisation
    }
    /// Gets a mutable reference to the module's organisation field.
    pub fn get_mut_organisation(&mut self) -> &mut String {
        &mut self.organisation
    }
    /// Gets a reference to the module's note field.
    pub fn get_note(&self) -> &String {
        &self.note
    }
    /// Gets a mutable reference to the module's note field.
    pub fn get_mut_note(&mut self) -> &mut String {
        &mut self.note
    }
    /// Gets a reference to the module's courses.
    pub fn get_courses(&self) -> &Vec<Course> {
        &self.courses
    }
    /// Gets a mutable reference to the module's courses.
    pub fn get_mut_courses(&mut self) -> &mut Vec<Course> {
        &mut self.courses
    }
    /// Gets a reference to the module's exams.
    pub fn get_exams(&self) -> &Vec<Exam> {
        &self.exams
    }
    /// Gets a mutable reference to the module's exams.
    pub fn get_mut_exams(&mut self) -> &mut Vec<Exam> {
        &mut self.exams
    }
    /// Gets a reference to the module's module_level field.
    pub fn get_module_level(&self) -> &String {
        &self.module_level
    }
    /// Gets a mutable reference to the module's module_level field.
    pub fn get_mut_module_level(&mut self) -> &mut String {
        &mut self.module_level
    }
    /// Gets a reference to the module's abbreviation field.
    pub fn get_abbreviation(&self) -> &String {
        &self.abbreviation
    }
    /// Gets a mutable reference to the module's abbreviation field.
    pub fn get_mut_abbreviation(&mut self) -> &mut String {
        &mut self.abbreviation
    }
    /// Gets a reference to the module's subtitle field.
    pub fn get_subtitle(&self) -> &String {
        &self.subtitle
    }
    /// Gets a mutable reference to the module's subtitle field.
    pub fn get_mut_subtitle(&mut self) -> &mut String {
        &mut self.subtitle
    }
    /// Gets a reference to the module's duration field.
    pub fn get_duration(&self) -> &String {
        &self.duration
    }
    /// Gets a mutable reference to the module's duration field.
    pub fn get_mut_duration(&mut self) -> &mut String {
        &mut self.duration
    }
    /// Gets a reference to the module's occurence field.
    pub fn get_occurence(&self) -> &String {
        &self.occurence
    }
    /// Gets a mutable reference to the module's occurence field.
    pub fn get_mut_occurence(&mut self) -> &mut String {
        &mut self.occurence
    }
    /// Gets a reference to the module's name.
    pub fn get_name(&self) -> &String {
        &self.name
    }
    /// Gets a mutable reference to the module's name.
    pub fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }
    /// Gets a reference to the module's id.
    pub fn get_id(&self) -> &String {
        &self.id
    }
    /// Gets a mutable reference to the module's id.
    pub fn get_mut_id(&mut self) -> &mut String {
        &mut self.id
    }
    /// Gets a mutable reference to the module's id.
    pub fn get_related_programs(&self) -> &String {
        &self.related_programs
    }
    /// Gets a mutable reference to the module's related_programs field.
    pub fn get_mut_related_programs(&mut self) -> &mut String {
        &mut self.related_programs
    }
    /// Gets a reference to the module's total_hours field.
    pub fn get_total_hours(&self) -> &Duration {
        &self.total_hours
    }
    /// Gets a mutable reference to the module's total_hours field.
    pub fn get_mut_total_hours(&mut self) -> &mut Duration {
        &mut self.total_hours
    }
    /// Gets a reference to the module's contact_hours field.
    pub fn get_contact_hours(&self) -> &Duration {
        &self.contact_hours
    }
    /// Gets a mutable reference to the module's contact_hours field.
    pub fn get_mut_contact_hours(&mut self) -> &mut Duration {
        &mut self.contact_hours
    }
    /// Gets a reference to the module's self_study_hours field.
    pub fn get_self_study_hours(&self) -> &Duration {
        &self.self_study_hours
    }
    /// Gets a mutable reference to the module's self_study_hours field.
    pub fn get_mut_self_study_hours(&mut self) -> &mut Duration {
        &mut self.self_study_hours
    }
    /// Gets a reference to the module's descr_of_achievement_assessment_methods field.
    pub fn get_descr_of_achievement_assessment_methods(&self) -> &String {
        &self.descr_of_achievement_assessment_methods
    }
    /// Gets a mutable reference to the module's descr_of_achievement_assessment_methods field.
    pub fn get_mut_descr_of_achievement_assessment_methods(&mut self) -> &mut String {
        &mut self.descr_of_achievement_assessment_methods
    }
    /// Gets a reference to the module's exam_retake_next_semester field.
    pub fn get_exam_retake_next_semester(&self) -> &bool {
        &self.exam_retake_next_semester
    }
    /// Gets a mutable reference to the module's exam_retake_next_semester field.
    pub fn get_mut_exam_retake_next_semester(&mut self) -> &mut bool {
        &mut self.exam_retake_next_semester
    }
    /// Gets a reference to the module's exam_retake_end_semester field.
    pub fn get_exam_retake_end_semester(&self) -> &bool {
        &self.exam_retake_end_semester
    }
    /// Gets a mutable reference to the module's exam_retake_end_semester field.
    pub fn get_mut_exam_retake_end_semester(&mut self) -> &mut bool {
        &mut self.exam_retake_end_semester
    }
    /// Gets a reference to the module's prerequisites field.
    pub fn get_prerequisites(&self) -> &String {
        &self.prerequisites
    }
    /// Gets a mutable reference to the module's prerequisites field.
    pub fn get_mut_prerequisites(&mut self) -> &mut String {
        &mut self.prerequisites
    }
    /// Gets a reference to the module's intended_learning_outcomes field.
    pub fn get_intended_learning_outcomes(&self) -> &String {
        &self.intended_learning_outcomes
    }
    /// Gets a mutable reference to the module's intended_learning_outcomes field.
    pub fn get_mut_intended_learning_outcomes(&mut self) -> &mut String {
        &mut self.intended_learning_outcomes
    }
    /// Gets a reference to the module's content field.
    pub fn get_content(&self) -> &String {
        &self.content
    }
    /// Gets a mutable reference to the module's content field.
    pub fn get_mut_content(&mut self) -> &mut String {
        &mut self.content
    }
    /// Gets a reference to the module's teaching_and_learning_methods field.
    pub fn get_teaching_and_learning_methods(&self) -> &String {
        &self.teaching_and_learning_methods
    }
    /// Gets a mutable reference to the module's teaching_and_learning_methods field.
    pub fn get_mut_teaching_and_learning_methods(&mut self) -> &mut String {
        &mut self.teaching_and_learning_methods
    }
    /// Gets a reference to the module's media field.
    pub fn get_media(&self) -> &String {
        &self.media
    }
    /// Gets a mutable reference to the module's media field.
    pub fn get_mut_media(&mut self) -> &mut String {
        &mut self.media
    }
    /// Gets a reference to the module's reading_list field.
    pub fn get_reading_list(&self) -> &String {
        &self.reading_list
    }
    /// Gets a mutable reference to the module's reading_list field.
    pub fn get_mut_reading_list(&mut self) -> &mut String {
        &mut self.reading_list
    }
    /// Gets a reference to the module's responsible_bis field.
    pub fn get_responsible_bis(&self) -> &String {
        &self.responsible_bis
    }
    /// Gets a mutable reference to the module's responsible_bis field.
    pub fn get_mut_responsible_bis(&mut self) -> &mut String {
        &mut self.responsible_bis
    }
}
