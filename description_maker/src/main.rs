use std::vec;

use serde::{Deserialize, Serialize};
use serde_yaml;
use tera;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Interviewee {
    full_name: String,
    short_name: String,
    title: String,
    website_url: Option<String>,
    email: Option<String>,
    x_username: Option<String>,
    linkedin_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Organization {
    name: String,
    website_url: Option<String>,
    x_username: Option<String>,
    linkedin_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Link {
    href: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EpisodeInfo {
    title: String,
    number: Option<u16>,
    description: String,
    other_urls: Option<Vec<Link>>,
    interviewees: Vec<Interviewee>,
    organizations: Vec<Organization>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PodcastInfo {
    website_url: Option<String>,
    x_username: Option<String>,
    linkedin_url: Option<String>,
    youtube_url: Option<String>,
    spotify_url: Option<String>,
    apple_podcasts_url: Option<String>,
    rss_url: Option<String>,
    transcript_site_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Context {
    episode_info: EpisodeInfo,
    podcast_info: PodcastInfo,
}

struct LinkFilter;

impl tera::Filter for LinkFilter {
    fn filter(
        &self,
        value: &tera::Value,
        args: &std::collections::HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        let value = value
            .as_str()
            .ok_or_else(|| tera::Error::msg("value is not a string"))?;

        let href = args
            .get("href")
            .ok_or_else(|| tera::Error::msg("Missing href argument"))?;
        let href = href
            .as_str()
            .ok_or_else(|| tera::Error::msg("href argument is not a string"))?;

        let type_ = args
            .get("type")
            .ok_or_else(|| tera::Error::msg("Missing type argument"))?;
        let type_ = type_
            .as_str()
            .ok_or_else(|| tera::Error::msg("type argument is not a string"))?;

        let result = match type_ {
            "html" => {
                format!("<a href=\"{}\">{}</a>", href, value)
            }
            "markdown" => {
                format!("[{}]({})", value, href)
            }
            "text" => {
                format!("{}: {}", value, href)
            }
            _ => return Err(tera::Error::msg(format!("Unknown type: {}", type_))),
        };
        Ok(tera::to_value(result)?)
    }
}

struct EmailFilter;

impl tera::Filter for EmailFilter {
    fn filter(
        &self,
        value: &tera::Value,
        args: &std::collections::HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        let email = args
            .get("address")
            .ok_or_else(|| tera::Error::msg("Missing address argument"))?;
        let mut email = email
            .as_str()
            .ok_or_else(|| tera::Error::msg("email address is not a string"))?
            .to_string();

        let type_ = args
            .get("type")
            .ok_or_else(|| tera::Error::msg("Missing type argument"))?;
        let type_ = type_
            .as_str()
            .ok_or_else(|| tera::Error::msg("type argument is not a string"))?;
        if type_ != "text" {
            email = format!("mailto:{}", email);
        }

        let mut args2 = args.clone();
        args2.insert("href".to_string(), email.into());

        let link = LinkFilter;
        return LinkFilter::filter(&link, value, &args2);
    }
}

const MY_TEMPLATE: &'static str = "

{%- set type='markdown' -%}

{{ episode_info.description | trim}}


Episode links:

{% if podcast_info.transcript_site_url -%}
{% if episode_info.number -%}
    {%- set title_slug = episode_info.number ~ ' ' ~ episode_info.title| slugify -%}
{% else -%}
    {%- set title_slug = episode_info.title | slugify -%}
{% endif -%}
{% set transcript_url = [podcast_info.transcript_site_url, title_slug] | join(sep=\"/\") -%}
- {{ 'Transcript' | link(href=transcript_url, type=type)}}
{% endif -%}

{%- for link in episode_info.other_urls -%}
- {{ link.text | link(href=link.href, type=type) }}
{% endfor -%}

{% for interviewee in episode_info.interviewees -%}

{%- if interviewee.website_url -%}
- {{ interviewee.short_name ~ \"'s website\" | link(href=interviewee.website_url, type=type) }}
{% endif -%}

{%- if interviewee.email -%}
- {{ interviewee.short_name ~ \"'s email\" | email(address=interviewee.email, type=type) }}
{% endif -%}

{%- if interviewee.x_username -%}
- {{ interviewee.short_name ~ \"'s X\" | link(href=\"https://twitter.com/\" ~ interviewee.x_username, type=type) }}
{% endif -%}

{%- if interviewee.linkedin_url -%}
- {{ interviewee.short_name ~ \"'s LinkedIn\" | link(href=interviewee.linkedin_url, type=type) }}
{% endif -%}

{%- endfor -%}


{% for organization in episode_info.organizations -%}

{%- if organization.website_url -%}
- {{ organization.name ~ \"'s website\" | link(href=organization.website_url, type=type) }}
{% endif -%}

{%- if organization.x_username -%}
- {{ organization.name ~ \"'s X\" | link(href=\"https://twitter.com/\" ~ organization.x_username, type=type) }}
{% endif -%}

{%- if organization.linkedin_url -%}
- {{ organization.name ~ \"'s LinkedIn\" | link(href=organization.linkedin_url, type=type) }}
{% endif -%}

{%- endfor %}

Podcast info:

{% if podcast_info.website_url -%}
- {{ 'Website' | link(href=podcast_info.website_url, type=type) }}
{% endif -%}

{% if podcast_info.x_username -%}
- {{ 'X' | link(href=\"https://twitter.com/\" ~ podcast_info.x_username, type=type) }}
{% endif -%}

{% if podcast_info.linkedin_url -%}
- {{ 'LinkedIn' | link(href=podcast_info.linkedin_url, type=type) }}
{% endif -%}

{% if podcast_info.youtube_url -%}
- {{ 'YouTube' | link(href=podcast_info.youtube_url, type=type) }}
{% endif -%}

{% if podcast_info.spotify_url -%}
- {{ 'Spotify' | link(href=podcast_info.spotify_url, type=type) }}
{% endif -%}

{% if podcast_info.apple_podcasts_url -%}
- {{ 'Apple Podcasts' | link(href=podcast_info.apple_podcasts_url, type=type) }}
{% endif -%}

{% if podcast_info.rss_url -%}
- {{ 'RSS' | link(href=podcast_info.rss_url, type=type) }}
{% endif -%}

";

fn main() {
    let podcast_info = PodcastInfo {
        website_url: Some("wwww.audrow.com".to_string()),
        x_username: Some("audrow".to_string()),
        linkedin_url: Some("https://www.linkedin.com/in/audrow/".to_string()),
        youtube_url: Some("www.foo.com".to_string()),
        spotify_url: Some("www.foo.com".to_string()),
        apple_podcasts_url: Some("www.foo.com".to_string()),
        rss_url: Some("www.foo.com".to_string()),
        transcript_site_url: Some("https://audrow-nash-podcast.github.io/transcripts".to_string()),
    };
    let context = Context {
        episode_info: EpisodeInfo {
            title: "My great title: the making".to_string(),
            number: Some(1),
            description: "My great description".to_string(),
            other_urls: Some(vec![
                Link {
                    href: "www.product.com".to_string(),
                    text: "Product".to_string(),
                },
                Link {
                    href: "www.docs.com".to_string(),
                    text: "Docs".to_string(),
                },
            ]),
            organizations: vec![Organization {
                name: "Rust Core Team".to_string(),
                website_url: Some("https://rust-lang.org".to_string()),
                x_username: Some("rustlang".to_string()),
                linkedin_url: None,
            }],
            interviewees: vec![
                Interviewee {
                    full_name: "Ashley Williams".to_string(),
                    short_name: "Ashley".to_string(),
                    title: "Rust Core Team".to_string(),
                    website_url: None,
                    email: Some("aw@gmail.com".to_string()),
                    x_username: Some("ag_dubs".to_string()),
                    linkedin_url: None,
                },
                Interviewee {
                    full_name: "Yoshua Wuyts".to_string(),
                    short_name: "Yosh".to_string(),
                    title: "CEO".to_string(),
                    website_url: Some("www.audrow.com".to_string()),
                    email: Some("yoshu@gmail.com".to_string()),
                    x_username: None,
                    linkedin_url: Some("https://www.linkedin.com/in/audrow/".to_string()),
                },
            ],
        },
        podcast_info: podcast_info.clone(),
    };
    let context = tera::Context::from_serialize(context).unwrap();
    let mut tera = tera::Tera::default();
    tera.add_raw_template("ep", MY_TEMPLATE).unwrap();
    tera.register_filter("link", LinkFilter);
    tera.register_filter("email", EmailFilter);
    let rendered = tera.render("ep", &context).unwrap();
    println!("{}", rendered);

    /*
        Create a directory structure like this
        - episode.yaml
        - transcript.txt
        - timecodes.txt
    */
    let demo_episode_yaml = EpisodeInfo {
        title: "TITLE".to_string(),
        number: Some(123),
        description: "This is a multiline description.

It has multiple lines.

And is very interesting."
            .to_string(),
        other_urls: Some(vec![
            Link {
                href: "www.product.com".to_string(),
                text: "Product".to_string(),
            },
            Link {
                href: "www.docs.com".to_string(),
                text: "Docs".to_string(),
            },
        ]),
        interviewees: vec![Interviewee {
            full_name: "Audrow Nash".to_string(),
            short_name: "Audrow".to_string(),
            title: "Software Enginee".to_string(),
            email: Some("audrow@hey.com".to_string()),
            x_username: Some("audrow".to_string()),
            linkedin_url: Some("https://www.linkedin.com/in/audrow/".to_string()),
            website_url: None,
        }],
        organizations: vec![Organization {
            name: "Audrow Nash Podcast".to_string(),
            x_username: Some("audrow".to_string()),
            linkedin_url: Some("https://www.linkedin.com/in/audrow/".to_string()),
            website_url: None,
        }],
    };

    let yaml = serde_yaml::to_string(&demo_episode_yaml).unwrap();
    print!("{}", yaml);
    let episode_dir = std::path::Path::new("episode");
    let episode_yaml = episode_dir.join("episode.yaml");

    std::fs::create_dir_all(episode_dir).unwrap();
    std::fs::write(&episode_yaml, yaml).unwrap();

    // read the yaml file
    let yaml = std::fs::read_to_string(&episode_yaml).unwrap();
    let episode_info = serde_yaml::from_str::<EpisodeInfo>(&yaml).unwrap();

    let context = Context {
        episode_info: episode_info,
        podcast_info: podcast_info,
    };
    let context = tera::Context::from_serialize(context).unwrap();
    let rendered = tera.render("ep", &context).unwrap();
    println!("{}", rendered);
}
