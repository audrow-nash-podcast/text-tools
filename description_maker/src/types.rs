use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    pub text: String,
    pub href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timestamp {
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.hours, self.minutes, self.seconds
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeCode {
    pub text: String,
    pub timestamp: Timestamp,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Episode {
    pub title: String,
    pub number: u16,
    pub description: String,
    pub links: Vec<Link>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PodcastInfo {
    pub name: String,
    pub transcript_site_url: String,
    pub links: Vec<Link>,
}
