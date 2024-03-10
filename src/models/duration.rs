use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct Duration(pub std::time::Duration);

#[derive(Debug)]
pub enum DurationParseError {
    InvalidUnit,
    InvalidNumber,
    TooSmall,
}

impl std::error::Error for DurationParseError {}

impl Display for DurationParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DurationParseError::InvalidUnit => write!(f, "Invalid unit"),
            DurationParseError::InvalidNumber => write!(f, "Invalid number"),
            DurationParseError::TooSmall => write!(f, "Duration is too small to be parsed"),
        }
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut duration = self.0.as_secs();
        let mut result = String::new();

        if duration >= 60 * 60 * 24 * 365 {
            let years = duration / (60 * 60 * 24 * 365);
            duration -= years * 60 * 60 * 24 * 365;
            result.push_str(&format!("{} yrs. ", years));
        }

        if duration >= 60 * 60 * 24 * 7 {
            let weeks = duration / (60 * 60 * 24 * 7);
            duration -= weeks * 60 * 60 * 24 * 7;
            result.push_str(&format!("{} w. ", weeks));
        }

        if duration >= 60 * 60 * 24 {
            let days = duration / (60 * 60 * 24);
            duration -= days * 60 * 60 * 24;
            result.push_str(&format!("{} d. ", days));
        }

        if duration >= 60 * 60 {
            let hours = duration / (60 * 60);
            duration -= hours * 60 * 60;
            result.push_str(&format!("{} h. ", hours));
        }

        if duration >= 60 {
            let minutes = duration / 60;
            duration -= minutes * 60;
            result.push_str(&format!("{} m. ", minutes));
        }

        if duration > 0 {
            result.push_str(&format!("{} s. ", duration));
        }

        f.write_str(&result)
    }
}

impl FromStr for Duration {
    type Err = DurationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut duration = std::time::Duration::default();
        let mut number = String::new();
        let mut unit = String::new();

        for c in s.chars() {
            if c.is_numeric() {
                number.push(c);
            } else {
                unit.push(c);
            }
        }

        let number = number.parse::<u64>().map_err(|_| DurationParseError::InvalidNumber)?;
        let unit = match unit.as_str() {
            "s" => std::time::Duration::from_secs(number),
            "m" => std::time::Duration::from_secs(number * 60),
            "h" => std::time::Duration::from_secs(number * 60 * 60),
            "d" => std::time::Duration::from_secs(number * 60 * 60 * 24),
            "w" => std::time::Duration::from_secs(number * 60 * 60 * 24 * 7),
            "y" => std::time::Duration::from_secs(number * 60 * 60 * 24 * 365),
            _ => return Err(DurationParseError::InvalidUnit),
        };

        duration += unit;
        if duration.as_secs() < 10 {
            return Err(DurationParseError::TooSmall);
        }

        Ok(Duration(duration))
    }
}