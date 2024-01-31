use regex::Regex;

use crate::types::{OutlineEntry, TimeCode};

#[derive(Debug)]
pub enum ParseOutlineError {
    InvalidTimeCode(String),
    InvalidOutlineEntry(String),
}

impl std::fmt::Display for ParseOutlineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseOutlineError::InvalidTimeCode(time_code) => {
                write!(f, "Invalid time code: {}", time_code)
            }
            ParseOutlineError::InvalidOutlineEntry(entry) => {
                write!(f, "Invalid outline entry: {}", entry)
            }
        }
    }
}

impl std::error::Error for ParseOutlineError {}

pub fn parse_outline(outline: &str) -> Result<Vec<OutlineEntry>, ParseOutlineError> {
    let mut entries = vec![];

    let outline_entry_regex =
        Regex::new(r"([\d:]+) (.+)").expect("Outline entry capture regex is valid");

    for line in outline.lines() {
        let captures = outline_entry_regex
            .captures(line)
            .ok_or(ParseOutlineError::InvalidOutlineEntry(line.to_string()))?;
        let time_code = match TimeCode::from_str(&captures[1]) {
            Ok(time_code) => time_code,
            Err(e) => return Err(ParseOutlineError::InvalidTimeCode(e.to_string())),
        };
        // println!("time_code: {:?}", time_code);
        // let time_code = time_code.map_err(|_| ParseOutlineError::InvalidTimeCode(captures[1].to_string()))?;
        let text = &captures[2];
        if text.is_empty() {
            return Err(ParseOutlineError::InvalidOutlineEntry(line.to_string()));
        }

        entries.push(OutlineEntry {
            time_code,
            text: text.into(),
        });
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_outline() {
        let outline_entries: Vec<OutlineEntry> = vec![
            OutlineEntry {
                time_code: TimeCode {
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                },
                text: "Start".to_string(),
            },
            OutlineEntry {
                time_code: TimeCode {
                    hours: 0,
                    minutes: 1,
                    seconds: 10,
                },
                text: "Introducing Bradley and Luxonis".to_string(),
            },
            OutlineEntry {
                time_code: TimeCode {
                    hours: 0,
                    minutes: 17,
                    seconds: 24,
                },
                text: "Introducing Rae robot".to_string(),
            },
            OutlineEntry {
                time_code: TimeCode {
                    hours: 0,
                    minutes: 53,
                    seconds: 44,
                },
                text: "How RobotHub works".to_string(),
            },
            OutlineEntry {
                time_code: TimeCode {
                    hours: 1,
                    minutes: 4,
                    seconds: 13,
                },
                text: "Security on RobotHub".to_string(),
            },
            OutlineEntry {
                time_code: TimeCode {
                    hours: 1,
                    minutes: 31,
                    seconds: 59,
                },
                text: "Links to share".to_string(),
            },
        ];

        let outline_text = outline_entries
            .iter()
            .map(|entry| format!("{} {}", entry.time_code, entry.text))
            .collect::<Vec<String>>()
            .join("\n");

        let parsed_outline_entries: Vec<OutlineEntry> =
            parse_outline(&outline_text).expect("parse_outline should succeed ");

        assert_eq!(parsed_outline_entries.len(), outline_entries.len());

        for (i, entry) in parsed_outline_entries.iter().enumerate() {
            assert_eq!(
                entry.time_code, outline_entries[i].time_code,
                "entry: {:?}",
                entry
            );
            assert_eq!(entry.text, outline_entries[i].text, "entry: {:?}", entry);
        }
    }

    #[test]
    fn error_for_invalid_minutes() {
        let outline_text = "00:60:00 Introducing Bradley and Luxonis";
        let result = parse_outline(&outline_text);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ParseOutlineError::InvalidTimeCode(_)
        ));
    }

    #[test]
    fn error_for_invalid_seconds() {
        let outline_text = "00:01:60 Introducing Bradley and Luxonis";
        let result = parse_outline(&outline_text);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ParseOutlineError::InvalidTimeCode(_)
        ));
    }

    #[test]
    fn errors_for_no_text() {
        let outline_text = "00:01:00 ";
        let result = parse_outline(&outline_text);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ParseOutlineError::InvalidOutlineEntry(_)
        ));
    }
}
