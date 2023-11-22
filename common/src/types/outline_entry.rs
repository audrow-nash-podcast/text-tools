use crate::types::time_code::TimeCode;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct OutlineEntry {
    pub time_code: TimeCode,
    pub text: String,
}

impl Eq for OutlineEntry {}

impl Ord for OutlineEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.time_code > other.time_code {
            Ordering::Greater
        } else if self.time_code < other.time_code {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for OutlineEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
