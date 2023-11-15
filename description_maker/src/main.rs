use std::vec;

mod types;
use types::{EpisodeInfo, Interviewee, Link, Organization, PodcastInfo};

use serde::{Deserialize, Serialize};
use tera;


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Context {
    episode_info: EpisodeInfo,
    podcast_info: PodcastInfo,
}

fn main() {
    let podcast_info = PodcastInfo {
        website_url: Some("wwww.audrow.com".to_string()),
        x_username: Some("audrow".to_string()),
        linkedin_url: Some("https://www.linkedin.com/in/audrow".to_string()),
        youtube_url: Some("www.foo.com".to_string()),
        spotify_url: Some("www.foo.com".to_string()),
        apple_podcasts_url: Some("www.foo.com".to_string()),
        rss_url: Some("www.foo.com".to_string()),
        transcript_site_url: Some("https://audrow-nash-podcast.github.io/transcripts".to_string()),
    };
    let context = Context {
        episode_info: EpisodeInfo {
            title: "Rethinking Robotics: Electric Sheep's Journey to Safer, Smarter Machines".to_string(),
            number: Some(1),
            description: r#"
How would you make ChatGPT but for the physical world? That's what Electric Sheep Robotics is working on. It's an audacious goal, and to accomplish it, they're doing many things fundamentally different than most robotics companies.

You'll enjoy this interview if you're interested in how AI and robotics fit together in a real application and if you want to see a new robotics business model that will probably become very popular."#.to_string(),
            other_urls: None,
            organizations: vec![Organization {
                name: "Electric Sheep".to_string(),
                website_url: Some("https://sheeprobotics.ai/".to_string()),
                x_username: Some("sheeprobotics".to_string()),
                linkedin_url: Some("https://www.linkedin.com/company/electric-sheep-robotics".to_string()),
            }],
            interviewees: vec![
                Interviewee {
                    full_name: "Nag Murty".to_string(),
                    short_name: "Nag".to_string(),
                    title: "Founder & CEO".to_string(),
                    website_url: None,
                    email: Some("nag@electricsheep.company".to_string()),
                    x_username: Some("MurtyNag".to_string()),
                    linkedin_url: Some("https://www.linkedin.com/in/nag-murty-b003383".to_string()),
                },
                Interviewee {
                    full_name: "Mike Laskey".to_string(),
                    short_name: "Mike".to_string(),
                    title: "VP of Autonomy".to_string(),
                    website_url: None,
                    email: Some("michael.laskey@electricsheep.company".to_string()),
                    x_username: None,
                    linkedin_url: Some("https://www.linkedin.com/in/michael-laskey-4b087ba2".to_string()),
                },
            ],
        },
        podcast_info: podcast_info.clone(),
    };


    let context = tera::Context::from_serialize(context).unwrap();
    let mut tera = tera::Tera::default();

    let file_directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let templates_directory = file_directory.join("src/templates");
    tera.add_template_files(vec![
        (templates_directory.join("spotify.html.tera"), Some("spotify")),
        (templates_directory.join("transcript.md.tera"), Some("transcript")),
        (templates_directory.join("x.txt.tera"), Some("x")),
        (templates_directory.join("youtube.txt.tera"), Some("youtube")),
    ]).expect("Templates to be added");

    let rendered = tera.render("youtube", &context).unwrap();
    println!("{}", rendered);
    println!("{}", file!());
}

