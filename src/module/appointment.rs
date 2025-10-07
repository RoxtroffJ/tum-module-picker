//! The [Appointment] struct

use super::*;

/// An appointment with date and place.
#[derive(Debug, Clone)]
pub struct Appointment {
    start: PrimitiveDateTime,
    stop: PrimitiveDateTime,
    place: String,
}
// Getters and mutable getters for Appointment
impl Appointment {
    /// Gets a reference to the appointment's start date.
    pub fn get_start(&self) -> &PrimitiveDateTime {
        &self.start
    }
    /// Gets a mutable reference to the appointment's start date.
    pub fn get_mut_start(&mut self) -> &mut PrimitiveDateTime {
        &mut self.start
    }
    /// Gets a reference to the appointment's duration.
    pub fn get_stop(&self) -> &PrimitiveDateTime {
        &self.stop
    }
    /// Gets a mutable reference to the appointment's duration.
    pub fn get_mut_stop(&mut self) -> &mut PrimitiveDateTime {
        &mut self.stop
    }
    /// Gets a reference to the appointment's place.
    pub fn get_place(&self) -> &String {
        &self.place
    }
    /// Gets a mutable reference to the appointment's place.
    pub fn get_mut_place(&mut self) -> &mut String {
        &mut self.place
    }
}

impl Appointment {
    /// Creates a new appointment.
    pub fn new(start: PrimitiveDateTime, stop: PrimitiveDateTime, place: String) -> Self {
        Self {
            start,
            stop,
            place,
        }
    }

    /// Creates a new appointment from a starting and ending dates instead of duration.
    pub fn from_duration(
        start: PrimitiveDateTime,
        duration: Duration,
        place: String,
    ) -> Self {
        Self::new(start, start + duration, place)
    }

    /// Reads a succession of appointments and returns an iterator over these appointments.
    ///
    /// # Example
    /// ```
    /// use tum_module_picker::module::*;
    /// use std::io::Cursor;
    /// use time::Time;
    /// use time::Month::{self, *};
    /// use time::Date;
    /// use time::PrimitiveDateTime;
    ///
    /// let input = Cursor::new(
    /// "15Oct 2025
    /// Wednesday, 15:00 - 17:00
    /// MW 2050, Zeichen-/Hörsaal (5510.02.050)
    /// 22Oct 2025
    /// Wednesday, 15:00 - 17:00
    /// MW 2050, Zeichen-/Hörsaal (5510.02.050)
    /// 29Oct 2025
    /// Wednesday, 15:00 - 17:00
    /// MW 2050, Zeichen-/Hörsaal (5510.02.050)
    /// ");
    /// let mut appointments = Appointment::from_reader(input);
    ///
    /// let app1 = appointments.next();
    /// let app2 = appointments.next();
    /// let app3 = appointments.next();
    /// let app4 = appointments.next();
    /// 
    /// fn date(year: i32, month: Month, day: u8, hour: u8, minute: u8) -> PrimitiveDateTime {
    ///     Date::from_calendar_date(year, month, day).unwrap()
    ///         .with_time(Time::from_hms(hour, minute, 00).unwrap())
    /// }
    /// 
    /// match (app1, app2, app3, app4) {
    ///     (Some(app1), Some(app2), Some(app3), None) => {
    ///         assert_eq!(app1.get_start(), &date(2025, October, 15, 15, 00));
    ///         assert_eq!(app1.get_stop(),  &date(2025, October, 15, 17, 00));
    ///         assert_eq!(app1.get_place(), "MW 2050, Zeichen-/Hörsaal (5510.02.050)");
    /// 
    ///         assert_eq!(app2.get_start(), &date(2025, October, 22, 15, 00));
    ///         assert_eq!(app2.get_stop(),  &date(2025, October, 22, 17, 00));
    ///         assert_eq!(app2.get_place(), "MW 2050, Zeichen-/Hörsaal (5510.02.050)");
    /// 
    ///         assert_eq!(app3.get_start(), &date(2025, October, 29, 15, 00));
    ///         assert_eq!(app3.get_stop(),  &date(2025, October, 29, 17, 00));
    ///         assert_eq!(app3.get_place(), "MW 2050, Zeichen-/Hörsaal (5510.02.050)");
    ///     }
    ///     _ => unreachable!("3 appointments and only 3")
    /// }
    /// ```
    pub fn from_reader<T: BufRead>(reader: T) -> impl Iterator<Item = Self> {
        let lines = reader.lines().filter_map(|l| l.ok());
        struct Result<T: Iterator<Item = String>> {
            lines: T,
        }
        type SelfBis = Appointment;

        impl<T: Iterator<Item = String>> Iterator for Result<T> {
            type Item = SelfBis;

            fn next(&mut self) -> Option<Self::Item> {
                // First line

                let first_line = |lines: &mut T| {
                    for line in lines {
                        let line = line.as_ref();

                        static REGEX: LazyLock<Regex> =
                            LazyLock::new(|| Regex::new(r"(?x)(\d*)(...)\D*(\d*)").unwrap());
                        match REGEX.captures(line) {
                            Some(captures) => {
                                let day =
                                    match captures.get(1).and_then(|m| m.as_str().parse().ok()) {
                                        Some(val) => val,
                                        None => continue,
                                    };

                                let month = match captures.get(2).and_then(|m| match m.as_str() {
                                    "Jan" => Some(Month::January),
                                    "Feb" => Some(Month::February),
                                    "Mar" => Some(Month::March),
                                    "Apr" => Some(Month::April),
                                    "May" => Some(Month::May),
                                    "Jun" => Some(Month::June),
                                    "Jul" => Some(Month::July),
                                    "Aug" => Some(Month::August),
                                    "Sep" => Some(Month::September),
                                    "Oct" => Some(Month::October),
                                    "Dec" => Some(Month::December),
                                    _ => None,
                                }) {
                                    Some(month) => month,
                                    None => continue,
                                };

                                let year =
                                    match captures.get(3).and_then(|m| m.as_str().parse().ok()) {
                                        Some(val) => val,
                                        None => continue,
                                    };

                                match Date::from_calendar_date(year, month, day) {
                                    Ok(date) => return Some(date),
                                    Err(_) => (),
                                };
                            }
                            None => (),
                        }
                    }
                    None
                };
                let second_line = |lines: &mut T| {
                    static REGEX: LazyLock<Regex> =
                        LazyLock::new(|| Regex::new(r"(\d*):(\d*)").unwrap());

                    for line in lines {
                        let line = line.as_ref();

                        let mut iter = REGEX.captures_iter(line);
                        let fst = iter.next();
                        let snd = iter.next();

                        match (fst, snd) {
                            (Some(fst), Some(snd)) => {
                                let fst_hour =
                                    match fst.get(1).and_then(|h| h.as_str().parse().ok()) {
                                        Some(h) => h,
                                        None => continue,
                                    };
                                let fst_min = match fst.get(2).and_then(|h| h.as_str().parse().ok())
                                {
                                    Some(m) => m,
                                    None => continue,
                                };

                                let fst = match Time::from_hms(fst_hour, fst_min, 0) {
                                    Ok(t) => t,
                                    Err(_) => continue,
                                };

                                let snd_hour =
                                    match snd.get(1).and_then(|h| h.as_str().parse().ok()) {
                                        Some(h) => h,
                                        None => continue,
                                    };
                                let snd_min = match snd.get(2).and_then(|h| h.as_str().parse().ok())
                                {
                                    Some(m) => m,
                                    None => continue,
                                };

                                let snd = match Time::from_hms(snd_hour, snd_min, 0) {
                                    Ok(t) => t,
                                    Err(_) => continue,
                                };

                                return Some((fst, snd));
                            }
                            _ => continue,
                        };
                    }
                    None
                };
                first_line(&mut self.lines).and_then(|date| match second_line(&mut self.lines) {
                    Some((start, stop)) => Some(Appointment::new(
                        date.with_time(start),
                        date.with_time(stop),
                        self.lines.next()?,
                    )),
                    None => None,
                })
            }
        }

        Result { lines }
    }
}
