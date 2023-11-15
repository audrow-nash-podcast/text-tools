use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interviewee {
    pub full_name: String,
    pub short_name: String,
    pub title: String,
    pub website_url: Option<String>,
    pub email: Option<String>,
    pub x_username: Option<String>,
    pub linkedin_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Organization {
    pub name: String,
    pub website_url: Option<String>,
    pub x_username: Option<String>,
    pub linkedin_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    pub href: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpisodeInfo {
    pub title: String,
    pub number: Option<u16>,
    pub description: String,
    pub other_urls: Option<Vec<Link>>,
    pub interviewees: Vec<Interviewee>,
    pub organizations: Vec<Organization>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PodcastInfo {
    pub website_url: Option<String>,
    pub x_username: Option<String>,
    pub linkedin_url: Option<String>,
    pub youtube_url: Option<String>,
    pub spotify_url: Option<String>,
    pub apple_podcasts_url: Option<String>,
    pub rss_url: Option<String>,
    pub transcript_site_url: Option<String>,
}
