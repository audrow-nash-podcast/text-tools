use std::fs;
use std::path::PathBuf;

use askama::Template;
use htmlescape;
use serde_yaml;

use crate::types::{Episode, Link, PodcastInfo};

use common::{parse_outline, OutlineEntry};

pub fn get_episode_slug(episode: &Episode) -> String {
    format!("{} {}", episode.number, episode.title)
        .to_lowercase()
        .replace(" ", "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
}

fn get_transcript_url(episode: &Episode, podcast_info: &PodcastInfo) -> String {
    let episode_slug = get_episode_slug(episode);
    format!("{}/{}.html", podcast_info.transcript_site_url, episode_slug)
}

fn prepare_html(text: &str) -> String {
    htmlescape::encode_minimal(text)
        .replace("\n", "<br/>")
        .replace("-", "&#8211;")
}

#[derive(Template, Clone)]
#[template(path = "spotify.html")]
struct SpotifyTemplate {
    episode: Episode,
    podcast_info: PodcastInfo,
    outline: Vec<OutlineEntry>,
}

#[derive(Template, Clone)]
#[template(path = "content.md")]
struct ContentTemplate {
    episode: Episode,
    podcast_info: PodcastInfo,
    spotify_html: String,
    outline: Vec<OutlineEntry>,
}

pub fn make_podcast_info_starter(save_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(save_dir).exists() {
        std::fs::create_dir_all(save_dir)?;
    }
    let podcast_info = PodcastInfo {
        name: "Your great podcast".to_string(),
        transcript_site_url: "https://www.ygp.com/transcripts".to_string(),
        links: vec![
            Link {
                text: "LinkedIn".to_string(),
                href: "https://www.linkedin.com/in/ygp/".to_string(),
            },
            Link {
                text: "Website".to_string(),
                href: "https://www.ygp.com/".to_string(),
            },
        ],
    };

    let podcast_yaml = serde_yaml::to_string(&podcast_info)?;
    let podcast_file_path = format!("{}/podcast.yaml", save_dir);
    std::fs::write(podcast_file_path, podcast_yaml)?;

    Ok(())
}

pub fn make_episode_starter(
    save_dir: &str,
    episode_file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(save_dir).exists() {
        std::fs::create_dir_all(save_dir)?;
    }
    let episode = Episode {
        description: "Your great episode\non multiple lines.".to_string(),
        title: "Hello, world!".to_string(),
        number: 1,
        links: vec![Link {
            text: "Company's LinkedIn".to_string(),
            href: "https://www.company.com/".to_string(),
        }],
    };

    let episode_yaml = serde_yaml::to_string(&episode)?;
    let episode_file_path = format!("{}/{}", save_dir, episode_file_name);
    std::fs::write(episode_file_path, episode_yaml)?;

    Ok(())
}

pub fn generate_content_markdown(
    podcast_path: &PathBuf,
    episode_path: &PathBuf,
    outline_path: &PathBuf,
    out_file_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let podcast_info = serde_yaml::from_str::<PodcastInfo>(&fs::read_to_string(podcast_path)?)?;
    let episode = serde_yaml::from_str::<Episode>(&fs::read_to_string(episode_path)?)?;
    let outline = parse_outline(&fs::read_to_string(outline_path)?)?;

    let template = SpotifyTemplate {
        episode: episode.clone(),
        podcast_info: podcast_info.clone(),
        outline: outline.clone(),
    };

    let spotify_html = template.render()?;
    let spotify_html = prepare_html(&spotify_html);

    let content_template = ContentTemplate {
        episode: episode.clone(),
        podcast_info: podcast_info.clone(),
        spotify_html,
        outline: outline.clone(),
    };
    let content_md = content_template.render()?;
    std::fs::write(out_file_path, content_md)?;

    Ok(())
}
