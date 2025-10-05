use super::*;

#[macro_export]
macro_rules! editable_maker {
    ($pub:vis, $name:ident, $($fields:ident),* ; $($non_str_fields_str:ident $non_str_fields_err:ident $err:ty),* ; $($others:ident $type:ty);*) => {
        #[derive(Debug)]
        $pub struct $name {
            $(pub $fields: bool,) *

            $(pub $non_str_fields_str: String, pub $non_str_fields_err: Option<$err>,) *

            $(pub $others: $type,) *
        }

        impl $name {
            pub fn with_all(self, value: bool) -> Self {
                Self {
                    $($fields: value,) *
                    ..self
                }
            }

            pub fn set_all(&mut self, value: bool) {
                $(self.$fields = value;) *
            }

            pub fn has_one_editable(&self) -> bool {
                $(self.$fields || )*false
            }
        }
    };
}

editable_maker!{
    pub,
    Editable,
    name, id,
    courses, exams,
    module_level, abbreviation, subtitle, duration, occurence, language, related_programs,
    total_hours, contact_hours, self_study_hours, 
    descr_of_achievement_assessment_methods,
    exam_retake_next_semester, exam_retake_end_semester, prerequisites, 
    intended_learning_outcomes, content, teaching_and_learning_methods, media, reading_list, 
    responsible_bis ;

    ;

}

impl Editable {
    /// Creates a new editable.
    ///
    /// Takes a reference to module to initialize non string [Module] fields.
    pub fn new() -> Self {
        Self {
            name: false,
            id: false,
            courses: false,
            exams: false,
            module_level: false,
            abbreviation: false,
            subtitle: false,
            duration: false,
            occurence: false,
            language: false,
            related_programs: false,
            total_hours: false,
            contact_hours: false,
            self_study_hours: false,
            descr_of_achievement_assessment_methods: false,
            exam_retake_next_semester: false,
            exam_retake_end_semester: false,
            prerequisites: false,
            intended_learning_outcomes: false,
            content: false,
            teaching_and_learning_methods: false,
            media: false,
            reading_list: false,
            responsible_bis: false,
        }
    }
}

/// When given an [Editable] option and one or multiple error fields, indicates if there is an error among them.
#[macro_export]
macro_rules! is_error {
    ($editable:expr) => {
        {false}
    };
    ($editable:expr, $($error_field:ident),+) => {{
        match $editable {
            Some(editable) => {$(editable.$error_field.is_some())||*},
            None => false
        }
    }};
}

/// When given an [Editable] option and one or multiple fields, indicates if one of these fields is true.
#[macro_export]
macro_rules! is_editable {
    ($editable:expr) => {
        {false}
    };
    ($editable:expr, $($error_field:ident),+) => {{
        match $editable {
            Some(editable) => {$(editable.$error_field)||*},
            None => false
        }
    }};
}

/// When given an [Editable] option and one or multiple fields, indicates if one of these fields is true. In this case, returns an option with the given value.
#[macro_export]
macro_rules! if_is_editable_opt {
    ($editable:expr, $val:expr) => {
        {false}
    };
    ($editable:expr, $val:expr, $($error_field:ident),+) => {{
        match $editable {
            Some(editable) => if $(editable.$error_field)||* {
                Some($val)
            } else {None},
            None => None
        }
    }};
}

/// When given an [Editable] option and fields, performs an if then and optionnaly else.
#[macro_export]
macro_rules! if_is_editable {
    ($editable:expr, $val:expr => $then:tt) => {
        if let Some(editable) = $editable {
            $then
        }
    };
    ($editable:expr, $val:expr, $($field:ident),+ => $then:tt) => {
        if let Some(editable) = $editable && ($(editable.$field)||+) {
            let ($($field),+) = ($(editable.$field),+);
            $then
        }
    };
    ($editable:expr, $val:expr => $then:tt else $else:tt) => {
        if let Some(editable) = $editable {
            $then
        } else {
            $else
        }
    };
    ($editable:expr, $val:expr, $($field:ident),+ => $then:tt else $else:tt) => {{
        if let Some(editable) = $editable && ($(editable.$field)||+) {
            let ($($field),+) = ($(editable.$field),+);
            $then
        } else {
            $else
        }
    }};
}

#[macro_export]
macro_rules! set_str_field {
    ($module:expr, $editable:expr, $str:expr, $field: ident) => {{

        if let Some(editable) = $editable && editable.$field{
            $module.$field = $str
        };
    }};
}


#[macro_export]
macro_rules! set_non_str_field {
    ($module:expr, $editable:expr, $parsed:expr, $str:expr, $field: ident, $field_string:ident, $field_error:ident) => {{
        let parsed = $parsed;
        if let Some(editable) = $editable && editable.$field{
            editable.$field_string = $str;
            match parsed {
                Ok(val) => {
                    editable.$field_error = None;
                    $module.$field = val;
                }
                Err(err) => editable.$field_error = Some(err),
            }
        };
    }};
}
