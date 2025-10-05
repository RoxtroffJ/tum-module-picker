//! Things related to the [Semester] struct.

use std::{cmp::{max, min}, fmt::Display, str::FromStr};

use super::*;

/// A semester. For example, 2025 summer semester.
#[derive(Debug, Clone)]
pub struct Semester {
    year: Year,
    winter: bool,
}
// Getters and mutable getters for Semester
impl Semester {
    /// Creates a new semester.
    ///
    /// For a year x, winter semester x is the winter semester starting in Semptember x.
    pub fn new(year: Year, is_winter: bool) -> Self {
        Self {
            year,
            winter: is_winter,
        }
    }

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

    /// Reads the string and finds a semester description in it.
    /// 
    /// Returns the semester and the starting (inclusive) and ending (exclusive) indexes of the match.
    pub fn parse_with_index(str: &str) -> Result<(Self, usize, usize), ParseError> {
        let year_match = YEAR_REGEX.find(str).ok_or(ParseError::CouldNotFindYear)?;

        let year = str[year_match.range()].parse().map_err(|_| ParseError::CouldNotFindYear)?;

        let winter_match = WINTER_REGEX.find(str);
        let summer_match = SUMMER_REGEX.find(str);

        let (is_winter, season_match) = match (winter_match, summer_match) {
            (None, None) => return Err(ParseError::CouldNotFindSeason),
            (None, Some(summer_match)) => (false, summer_match),
            (Some(winter_match), None) => (true, winter_match),
            (Some(winter_match), Some(summer_match)) => {
                if winter_match.start() < summer_match.start() {
                    (true, winter_match)
                } else {
                    (false, summer_match)
                }
        }};

        Ok((
            Self::new(year, is_winter),
            min(year_match.start(), season_match.start()),
            max(year_match.end(), season_match.end())
        ))
        
    }
}

impl Display for Semester {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} semester {}",
            if self.winter { "winter" } else { "summer" },
            self.year
        )
    }
}

static YEAR_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());
static SUMMER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\b{start}(?:S|Summer)\b{end}").unwrap());
static WINTER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\b{start}(?:W|Winter)\b{end}").unwrap());

/// Error returned when [FromStr] fails on a [Semester].
#[derive(Debug, Clone, Copy)]
pub enum ParseError {
    /// Variant returned when the year of the semester could not be determined.
    CouldNotFindYear,
    /// Variant returned when the season (winter or summer) could not be determined.
    CouldNotFindSeason
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::CouldNotFindYear => write!(f, "Can't determine the year of the semester"),
            ParseError::CouldNotFindSeason => write!(f, "Can't determine if it is winter or summer semester"),
        }
    }
}

impl FromStr for Semester {
    type Err = ParseError;


    /// Parses a string. It must contain a number and either `w`, `winter`, `s`, `summer`.
    /// 
    /// The parsing is case insensitive.
    /// 
    /// Example
    /// ```
    /// use tum_module_picker::module::semester::Semester;
    /// use std::str::FromStr;
    /// 
    /// let parse1 = <Semester as FromStr>::from_str("W2025");
    /// let parse2 = <Semester as FromStr>::from_str("2025 sUmmEr");
    /// 
    /// assert!(parse1.is_ok());
    /// assert!(parse2.is_ok());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_with_index(s).map(|(res,_,_)| res)
    }
}