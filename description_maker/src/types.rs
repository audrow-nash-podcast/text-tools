use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    pub text: String,
    pub href: String,
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
