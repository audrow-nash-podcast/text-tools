#[derive(Clone, Debug)]
pub struct Link {
    pub text: String,
    pub href: String,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct TimeCode {
    pub text: String,
    pub timestamp: Timestamp,
}

#[derive(Clone, Debug)]
pub struct Episode {
    pub title: String,
    pub number: u16,
    pub description: String,
    pub links: Vec<Link>,
    pub time_codes: Vec<TimeCode>,
}

#[derive(Clone, Debug)]
pub struct PodcastInfo {
    pub name: String,
    pub transcript_site_url: String,
    pub links: Vec<Link>,
}
