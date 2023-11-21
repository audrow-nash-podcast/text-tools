use askama::Template; // bring trait in scope
use htmlescape;

mod types;
use types::{Episode, Link, PodcastInfo, TimeCode, Timestamp};

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
}

#[derive(Template, Clone)]
#[template(path = "content.md")]
struct ContentTemplate {
    episode: Episode,
    podcast_info: PodcastInfo,
    spotify_html: String,
}

fn main() {
    let podcast_info = PodcastInfo {
        name: "Audrow Nash Podcast".to_string(),
        transcript_site_url: "https://www.audrow.com".to_string(),
        links: vec![
            Link {
                text: "LinkedIn".to_string(),
                href: "https://www.linkedin.com/in/audrow/".to_string(),
            },
            Link {
                text: "Website".to_string(),
                href: "https://www.audrow.com/".to_string(),
            },
        ],
    };
    let episode = Episode {
        description: "A podcast about robots and stuff\n\nThe best stuff\n\nThe best".to_string(),
        title: "Hello, world!".to_string(),
        number: 1,
        links: vec![Link {
            text: "Full transcript".to_string(),
            href: "https://www.audrow.com/".to_string(),
        }],
        time_codes: vec![
            TimeCode {
                text: "Introduction".to_string(),
                timestamp: Timestamp {
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                },
            },
            TimeCode {
                text: "Nag and Mike introduce themselves".to_string(),
                timestamp: Timestamp {
                    hours: 0,
                    minutes: 1,
                    seconds: 30,
                },
            },
            TimeCode {
                text: "Wrapping up".to_string(),
                timestamp: Timestamp {
                    hours: 1,
                    minutes: 30,
                    seconds: 30,
                },
            },
        ],
    };

    let template = SpotifyTemplate {
        episode: episode.clone(),
        podcast_info: podcast_info.clone(),
    };

    let spotify_html = template.render().expect("Template renders");
    let spotify_html = prepare_html(&spotify_html);
    println!("{}", spotify_html);

    let content_template = ContentTemplate {
        episode: episode.clone(),
        podcast_info: podcast_info.clone(),
        spotify_html,
    };
    let content_md = content_template.render().expect("Template renders");
    println!("{}", content_md)
}
