use std::fs;

use askama::Template; // bring trait in scope
use htmlescape;
use serde_yaml;

mod types;
use types::{Episode, Link, PodcastInfo};

use common::{OutlineEntry, TimeCode};

fn get_episode_slug(episode: &Episode) -> String {
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

fn make_podcast_info_starter(save_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    let podcast_yaml = serde_yaml::to_string(&podcast_info).expect("Podcast to serialize");
    let podcast_file_path = format!("{}/podcast.yaml", save_dir);
    std::fs::write(podcast_file_path, podcast_yaml).expect("Podcast to write");

    Ok(())
}

fn make_episode_starter(save_dir: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    let episode_yaml = serde_yaml::to_string(&episode).expect("Episode to serialize");
    let episode_file_path = format!("{}/{}.yaml", save_dir, file_name);
    std::fs::write(episode_file_path, episode_yaml).expect("Episode to write");

    Ok(())
}

fn main() {
    let out_dir = "out";
    let file_name = "my_episode";
    //delete dir
    if std::path::Path::new(out_dir).exists() {
        std::fs::remove_dir_all(out_dir).expect("Dir to delete");
    }
    make_podcast_info_starter(out_dir).expect("Make podcast info");
    make_episode_starter(out_dir, file_name).expect("Make episode info");

    let podcast_info = serde_yaml::from_str::<PodcastInfo>(
        &fs::read_to_string(format!("{}/podcast.yaml", out_dir)).expect("Podcast to read"),
    )
    .expect("Podcast to deserialize");
    let episode = serde_yaml::from_str::<Episode>(
        &fs::read_to_string(format!("{}/{}.yaml", out_dir, file_name)).expect("Episode to read"),
    )
    .expect("Episode to deserialize");
    let outline = vec![
        OutlineEntry {
            text: "Introduction".to_string(),
            time_code: TimeCode {
                hours: 0,
                minutes: 0,
                seconds: 0,
            },
        },
        OutlineEntry {
            text: "Nag and Mike introduce themselves".to_string(),
            time_code: TimeCode {
                hours: 0,
                minutes: 1,
                seconds: 30,
            },
        },
        OutlineEntry {
            text: "Wrapping up".to_string(),
            time_code: TimeCode {
                hours: 1,
                minutes: 30,
                seconds: 30,
            },
        },
    ];

    let template = SpotifyTemplate {
        episode: episode.clone(),
        podcast_info: podcast_info.clone(),
        outline: outline.clone(),
    };

    let spotify_html = template.render().expect("Template renders");
    let spotify_html = prepare_html(&spotify_html);
    println!("{}", spotify_html);

    let content_template = ContentTemplate {
        episode: episode.clone(),
        podcast_info: podcast_info.clone(),
        spotify_html,
        outline: outline.clone(),
    };
    let content_md = content_template.render().expect("Template renders");
    println!("{}", content_md);
}
