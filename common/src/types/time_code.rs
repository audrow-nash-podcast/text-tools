use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::error::Error;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TimeCode {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl TimeCode {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Result<TimeCode, Box<dyn Error>> {
        if minutes > 59 {
            return Err("Minutes must be between 0-59".into());
        }
        if seconds > 59 {
            return Err("Seconds must be between 0-59".into());
        }
        Ok(TimeCode {
            hours,
            minutes,
            seconds,
        })
    }
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
        let groups = text
            .split(":")
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()?;
        if groups.len() == 3 {
            return TimeCode::new(groups[0], groups[1], groups[2]);
        } else if groups.len() == 2 {
            return TimeCode::new(0, groups[0], groups[1]);
        } else {
            return Err(format!("Invalid time code: {}", text).into());
        }
    }
}

impl std::fmt::Display for TimeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.hours > 0 {
            write!(f, "{}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
        } else {
            write!(f, "{}:{:02}", self.minutes, self.seconds)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn with_valid() {
            assert_eq!(
                TimeCode::new(0, 0, 0).unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::new(0, 0, 1).unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 1
                }
            );
            assert_eq!(
                TimeCode::new(0, 1, 0).unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 1,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::new(1, 0, 0).unwrap(),
                TimeCode {
                    hours: 1,
                    minutes: 0,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::new(1, 1, 1).unwrap(),
                TimeCode {
                    hours: 1,
                    minutes: 1,
                    seconds: 1
                }
            );
            assert_eq!(
                TimeCode::new(1, 59, 59).unwrap(),
                TimeCode {
                    hours: 1,
                    minutes: 59,
                    seconds: 59,
                }
            );
        }

        #[test]
        fn with_invalid() {
            assert!(TimeCode::new(0, 0, 60).is_err());
            assert!(TimeCode::new(0, 60, 0).is_err());
            assert!(TimeCode::new(0, 60, 60).is_err());
            assert!(TimeCode::new(0, 100, 200).is_err());
        }
    }

    mod from_string {

        use super::*;

        #[test]
        fn with_mmss() {
            assert_eq!(
                TimeCode::from_str("00:00").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::from_str("00:01").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 1
                }
            );
            assert_eq!(
                TimeCode::from_str("01:00").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 1,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::from_str("01:01").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 1,
                    seconds: 1
                }
            );
        }

        #[test]
        fn with_mss() {
            assert_eq!(
                TimeCode::from_str("0:00").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::from_str("0:01").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 1
                }
            );
            assert_eq!(
                TimeCode::from_str("1:00").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 1,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::from_str("1:01").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 1,
                    seconds: 1
                }
            );
        }

        #[test]
        fn with_hhmmss() {
            assert_eq!(
                TimeCode::from_str("00:00:00").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::from_str("00:00:01").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 1
                }
            );
            assert_eq!(
                TimeCode::from_str("00:01:00").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 1,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::from_str("01:00:00").unwrap(),
                TimeCode {
                    hours: 1,
                    minutes: 0,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::from_str("01:01:01").unwrap(),
                TimeCode {
                    hours: 1,
                    minutes: 1,
                    seconds: 1
                }
            );
        }

        #[test]
        fn from_str_with_hmmss() {
            assert_eq!(
                TimeCode::from_str("0:00:00").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::from_str("0:00:01").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 1
                }
            );
            assert_eq!(
                TimeCode::from_str("0:01:00").unwrap(),
                TimeCode {
                    hours: 0,
                    minutes: 1,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::from_str("1:00:00").unwrap(),
                TimeCode {
                    hours: 1,
                    minutes: 0,
                    seconds: 0
                }
            );
            assert_eq!(
                TimeCode::from_str("1:01:01").unwrap(),
                TimeCode {
                    hours: 1,
                    minutes: 1,
                    seconds: 1
                }
            );
        }

        #[test]
        fn with_invalid() {
            assert!(TimeCode::from_str("00:00:").is_err());
            assert!(TimeCode::from_str("00::00").is_err());
            assert!(TimeCode::from_str(":00:00").is_err());
            assert!(TimeCode::from_str("::").is_err());
            assert!(TimeCode::from_str("a:b:c").is_err());
            assert!(TimeCode::from_str("00:00:00:00").is_err());
            assert!(TimeCode::from_str("00:00:00:00:00").is_err());
        }
    }

    mod display {

        use super::*;

        #[test]
        fn displays_correctly() {
            assert_eq!(
                TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 0
                }
                .to_string(),
                "0:00"
            );
            assert_eq!(
                TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 1
                }
                .to_string(),
                "0:01"
            );
            assert_eq!(
                TimeCode {
                    hours: 0,
                    minutes: 1,
                    seconds: 0
                }
                .to_string(),
                "1:00"
            );
            assert_eq!(
                TimeCode {
                    hours: 0,
                    minutes: 10,
                    seconds: 0
                }
                .to_string(),
                "10:00"
            );
            assert_eq!(
                TimeCode {
                    hours: 1,
                    minutes: 0,
                    seconds: 0
                }
                .to_string(),
                "1:00:00"
            );
            assert_eq!(
                TimeCode {
                    hours: 1,
                    minutes: 1,
                    seconds: 1
                }
                .to_string(),
                "1:01:01"
            );
            assert_eq!(
                TimeCode {
                    hours: 10,
                    minutes: 10,
                    seconds: 10
                }
                .to_string(),
                "10:10:10"
            );
            assert_eq!(
                TimeCode {
                    hours: 100,
                    minutes: 59,
                    seconds: 59
                }
                .to_string(),
                "100:59:59"
            )
        }
    }
}
