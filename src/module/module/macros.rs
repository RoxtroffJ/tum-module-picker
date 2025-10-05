/// Gets a reference to the `field` of `v` from the [Field] enum.
#[macro_export]
macro_rules! module_get_fields {
    ($v:expr, $field:expr) => {{
        use tum_module_picker::module::Field::{self, *};
        let e: Field = $field;
        match e {
            // === Overview ===
            Name => &$v.name,
            Id => &$v.id,
            ECTS => &$v.ects,
            Version => &$v.version,
            ValidFrom => &$v.valid_from,
            ValidUntil => &$v.valid_until,
            Responsible => &$v.responsible,
            Organisation => &$v.organisation,
            Note => &$v.note,
            // === Courses and exams ===
            Courses => &$v.courses,
            Exams => &$v.exams,
            // === Description ===
            // General data
            ModuleLevel => &$v.module_level,
            Abbreviation => &$v.abbreviation,
            Subtitle => &$v.subtitle,
            Duration => &$v.duration,
            Occurence => &$v.occurence,
            Language => &$v.language,
            RelatedPrograms => &$v.related_programs,
            // Work load
            TotalHours => &$v.total_hours,
            ContactHours => &$v.contact_hours,
            SelfStudyHours => &$v.self_study_hours,
            // Study and examination performance
            DescrOfAchievementAssessmentMethods => &$v.descr_of_achievement_assessment_methods,
            ExamRetakeNextSemester => &$v.exam_retake_next_semester,
            ExamRetakeEndSemester => &$v.exam_retake_end_semester,
            // Description
            Prerequisites => &$v.prerequisites,
            IntendedLearningOutcomes => &$v.intended_learning_outcomes,
            Content => &$v.content,
            TeachingAndLearningMethods => &$v.teaching_and_learning_methods,
            Media => &$v.media,
            ReadingList => &$v.reading_list,
            // Responsible for module
            ResponsibleBis => &$v.responsible_bis,
        }
    }};
}

/// Same as [module_get_field] but returns a mutable reference.
#[macro_export]
macro_rules! module_get_mut_field {
    ($v:expr, $e:expr) => {{
        use tum_module_picker::module::module::Field::{self, *};
        let e: Field = $e;
        match e {
            // === Overview ===
            Name => $v.get_mut_name(),
            Id => $v.get_mut_id(),
            ECTS => $v.get_mut_ects(),
            Version => $v.get_mut_version(),
            ValidFrom => $v.get_mut_valid_from(),
            ValidUntil => $v.get_mut_valid_until(),
            Responsible => $v.get_mut_responsible(),
            Organisation => $v.get_mut_organisation(),
            Note => $v.get_mut_note(),
            // === Courses and exams ===
            Courses => $v.get_mut_courses(),
            Exams => $v.get_mut_exams(),
            // === Description ===
            // General data
            ModuleLevel => $v.get_mut_module_level(),
            Abbreviation => $v.get_mut_abbreviation(),
            Subtitle => $v.get_mut_subtitle(),
            Duration => $v.get_mut_duration(),
            Occurence => $v.get_mut_occurence(),
            Language => $v.get_mut_language(),
            RelatedPrograms => $v.get_mut_related_programs(),
            // Work load
            TotalHours => $v.get_mut_total_hours(),
            ContactHours => $v.get_mut_contact_hours(),
            SelfStudyHours => $v.get_mut_self_study_hours(),
            // Study and examination performance
            DescrOfAchievementAssessmentMethods => {
                $v.get_mut_descr_of_achievement_assessment_methods()
            }
            ExamRetakeNextSemester => $v.get_mut_exam_retake_next_semester(),
            ExamRetakeEndSemester => $v.get_mut_exam_retake_end_semester(),
            // Description
            Prerequisites => $v.get_mut_prerequisites(),
            IntendedLearningOutcomes => $v.get_mut_intended_learning_outcomes(),
            Content => $v.get_mut_content(),
            TeachingAndLearningMethods => $v.get_mut_teaching_and_learning_methods(),
            Media => $v.get_mut_media(),
            ReadingList => $v.get_mut_reading_list(),
            // Responsible for module
            ResponsibleBis => $v.get_mut_responsible_bis(),
        }
    }};
}

/// Gets the enum corresponding to the field.
#[macro_export]
macro_rules! module_into_enum {
    (name) => {
        Field::Name
    };
    (id) => {
        Field::Id
    };
    (ects) => {
        Field::ECTS
    };
    (version) => {
        Field::Version
    };
    (valid_from) => {
        Field::ValidFrom
    };
    (valid_until) => {
        Field::ValidUntil
    };
    (responsible) => {
        Field::Responsible
    };
    (organisation) => {
        Field::Organisation
    };
    (note) => {
        Field::Note
    };
    (courses) => {
        Field::Courses
    };
    (exams) => {
        Field::Exams
    };
    (module_level) => {
        Field::ModuleLevel
    };
    (abbreviation) => {
        Field::Abbreviation
    };
    (subtitle) => {
        Field::Subtitle
    };
    (duration) => {
        Field::Duration
    };
    (occurence) => {
        Field::Occurence
    };
    (language) => {
        Field::Language
    };
    (related_programs) => {
        Field::RelatedPrograms
    };
    (total_hours) => {
        Field::TotalHours
    };
    (contact_hours) => {
        Field::ContactHours
    };
    (self_study_hours) => {
        Field::SelfStudyHours
    };
    (descr_of_achievement_assessment_methods) => {
        Field::DescrOfAchievementAssessmentMethods
    };
    (exam_retake_next_semester) => {
        Field::ExamRetakeNextSemester
    };
    (exam_retake_end_semester) => {
        Field::ExamRetakeEndSemester
    };
    (prerequisites) => {
        Field::Prerequisites
    };
    (intended_learning_outcomes) => {
        Field::IntendedLearningOutcomes
    };
    (content) => {
        Field::Content
    };
    (teaching_and_learning_methods) => {
        Field::TeachingAndLearningMethods
    };
    (media) => {
        Field::Media
    };
    (reading_list) => {
        Field::ReadingList
    };
    (responsible_bis) => {
        Field::ResponsibleBis
    };
}
