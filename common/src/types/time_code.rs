use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::error::Error;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
            Regex::new(r"^((\d+):)?(\d{2}):(\d{2})$").expect("Time code regex is valid");

        let captures = time_code_regex.captures(text).ok_or("Invalid time code")?;

        let hours = captures
            .get(2)
            .map_or(Ok(0), |m| m.as_str().parse::<u32>())?;
        let minutes = captures[3].parse::<u32>()?;
        let seconds = captures[4].parse::<u32>()?;

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
            "{}:{:02}:{:02}",
            if self.hours < 100 {
                format!("{:02}", self.hours)
            } else {
                self.hours.to_string()
            },
            self.minutes,
            self.seconds
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_two_entries() {
        assert_eq!(
            TimeCode::from_str("00:00").expect("Time code should parse"),
            TimeCode {
                hours: 0,
                minutes: 0,
                seconds: 0,
            }
        );
        assert_eq!(
            TimeCode::from_str("00:01").expect("Time code should parse"),
            TimeCode {
                hours: 0,
                minutes: 0,
                seconds: 1,
            }
        );
        assert_eq!(
            TimeCode::from_str("01:00").expect("Time code should parse"),
            TimeCode {
                hours: 0,
                minutes: 1,
                seconds: 0,
            }
        );
        assert_eq!(
            TimeCode::from_str("01:02").expect("Time code should parse"),
            TimeCode {
                hours: 0,
                minutes: 1,
                seconds: 2,
            }
        );
    }

    #[test]
    fn from_str_three_entries() {
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
            TimeCode::from_str("01:02:03").expect("Time code should parse"),
            TimeCode {
                hours: 1,
                minutes: 2,
                seconds: 3,
            }
        );
    }

    #[test]
    fn from_str_different_hours() {
        assert_eq!(
            TimeCode::from_str("1:22:33").expect("Time code should parse"),
            TimeCode {
                hours: 1,
                minutes: 22,
                seconds: 33,
            }
        );
        assert_eq!(
            TimeCode::from_str("01:22:33").expect("Time code should parse"),
            TimeCode {
                hours: 1,
                minutes: 22,
                seconds: 33,
            }
        );
        assert_eq!(
            TimeCode::from_str("001:22:33").expect("Time code should parse"),
            TimeCode {
                hours: 1,
                minutes: 22,
                seconds: 33,
            }
        );
        assert_eq!(
            TimeCode::from_str("12:22:33").expect("Time code should parse"),
            TimeCode {
                hours: 12,
                minutes: 22,
                seconds: 33,
            }
        );
        assert_eq!(
            TimeCode::from_str("1234:22:33").expect("Time code should parse"),
            TimeCode {
                hours: 1234,
                minutes: 22,
                seconds: 33,
            }
        );
        assert_eq!(
            TimeCode::from_str("000001234:22:33").expect("Time code should parse"),
            TimeCode {
                hours: 1234,
                minutes: 22,
                seconds: 33,
            }
        );
    }

    #[test]
    fn error_for_minutes_overflow() {
        let result = TimeCode::from_str("00:60:00");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Minutes must be between 0-59"
        );
    }

    #[test]
    fn error_for_seconds_overflow() {
        let result = TimeCode::from_str("00:01:60");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Seconds must be between 0-59"
        );
    }

    #[test]
    fn error_for_wrong_minute_format() {
        let result = TimeCode::from_str("00:1:00");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid time code");
    }

    #[test]
    fn error_for_wrong_second_format() {
        let result = TimeCode::from_str("00:00:1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid time code");
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
