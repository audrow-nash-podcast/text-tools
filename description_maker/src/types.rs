use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    pub text: String,
    pub href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Noun {
    pub name: String,
    pub x_handle: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpisodeInfo {
    pub title: String,
    pub number: u16,
    pub description: String,
    pub links: Vec<Link>,
    pub guests: Vec<Noun>,
    pub organization: Option<Noun>,
    pub youtube_video_url: Option<String>,
    pub x_post_url: Option<String>,
    pub spotify_for_podcasters_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PodcastInfo {
    pub name: String,
    pub transcript_site_url: String,
    pub links: Vec<Link>,
}
