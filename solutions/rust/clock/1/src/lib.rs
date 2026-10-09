use std::{fmt, str::FromStr};

const MINUTES_PER_DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = (hours * 60 + minutes).rem_euclid(MINUTES_PER_DAY);
        Self { minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

#[derive(Debug)]
pub enum ParseClockError {
    InvalidTimeFormat,
    InvalidHours,
    InvalidMinutes,
}

impl fmt::Display for ParseClockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTimeFormat => write!(f, "Invalid time format, expected HH:MM"),
            Self::InvalidHours => write!(f, "Invalid hours"),
            Self::InvalidMinutes => write!(f, "Invalid minutes"),
        }
    }
}

impl std::error::Error for ParseClockError {}

impl FromStr for Clock {
    type Err = ParseClockError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        let split = trimmed.split(':').collect::<Vec<&str>>();
        if split.len() != 2 {
            return Err(ParseClockError::InvalidTimeFormat);
        }
        let hours = split[0]
            .parse::<i32>()
            .map_err(|_| ParseClockError::InvalidHours)?;
        let minutes = split[1]
            .parse::<i32>()
            .map_err(|_| ParseClockError::InvalidMinutes)?;
        Ok(Self::new(hours, minutes))
    }
}
