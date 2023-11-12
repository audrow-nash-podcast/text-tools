use regex::Regex;
use std::cmp::Ordering;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct TimeCode {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl PartialOrd for TimeCode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hours > other.hours {
            Some(Ordering::Greater)
        } else if self.hours < other.hours {
            Some(Ordering::Less)
        } else if self.minutes > other.minutes {
            Some(Ordering::Greater)
        } else if self.minutes < other.minutes {
            Some(Ordering::Less)
        } else if self.seconds > other.seconds {
            Some(Ordering::Greater)
        } else if self.seconds < other.seconds {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl TimeCode {
    pub fn from_str(text: &str) -> Result<TimeCode, Box<dyn Error>> {
        let time_code_regex =
            Regex::new(r"(\d{2,}):(\d{2}):(\d{2})").expect("Time code regex is valid");
        let captures = time_code_regex.captures(text).ok_or("Invalid time code")?;

        let hours = u32::from_str(&captures[1])?;
        let minutes = u32::from_str(&captures[2])?;
        let seconds = u32::from_str(&captures[3])?;

        if minutes > 59 {
            return Err("Minutes must be between 0-59".into());
        }
        if seconds > 59 {
            return Err("Seconds must be between 0-59".into());
        }

        return Ok(TimeCode {
            hours,
            minutes,
            seconds,
        });
    }
}

impl std::fmt::Display for TimeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.hours, self.minutes, self.seconds
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(
            TimeCode::from_str("00:00:00").expect("Time code should parse"),
            TimeCode {
                hours: 0,
                minutes: 0,
                seconds: 0,
            }
        );
        assert_eq!(
            TimeCode::from_str("00:00:01").expect("Time code should parse"),
            TimeCode {
                hours: 0,
                minutes: 0,
                seconds: 1,
            }
        );
        assert_eq!(
            TimeCode::from_str("00:01:00").expect("Time code should parse"),
            TimeCode {
                hours: 0,
                minutes: 1,
                seconds: 0,
            }
        );
        assert_eq!(
            TimeCode::from_str("01:00:00").expect("Time code should parse"),
            TimeCode {
                hours: 1,
                minutes: 0,
                seconds: 0,
            }
        );
        assert_eq!(
            TimeCode::from_str("01:01:01").expect("Time code should parse"),
            TimeCode {
                hours: 1,
                minutes: 1,
                seconds: 1,
            }
        );
    }

    #[test]
    fn equality() {
        let t1 = TimeCode {
            hours: 1,
            minutes: 2,
            seconds: 3,
        };

        let t2 = TimeCode {
            hours: 1,
            minutes: 2,
            seconds: 3,
        };

        assert_eq!(t1, t2);

        let t3 = TimeCode {
            hours: 1,
            minutes: 2,
            seconds: 4,
        };

        assert!(t1 != t3)
    }

    #[test]
    fn order_time_codes() {
        let t1 = TimeCode {
            hours: 0,
            minutes: 0,
            seconds: 0,
        };

        let t2 = TimeCode {
            hours: 0,
            minutes: 0,
            seconds: 1,
        };

        let t3 = TimeCode {
            hours: 0,
            minutes: 1,
            seconds: 0,
        };

        let t4 = TimeCode {
            hours: 1,
            minutes: 0,
            seconds: 0,
        };

        let t5 = TimeCode {
            hours: 1,
            minutes: 1,
            seconds: 1,
        };

        assert!(t1 < t2);
        assert!(t2 < t3);
        assert!(t3 < t4);
        assert!(t4 < t5);
    }
}
