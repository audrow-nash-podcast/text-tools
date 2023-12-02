use clap::{arg, command, value_parser, Command};
use std::path::PathBuf;

mod template;
use crate::template::{
    generate_content_markdown, make_episode_starter, make_outline_starter,
    make_podcast_info_starter,
};

mod types;

fn main() {
    let matches = cmd().get_matches();

    match matches.subcommand() {
        Some(("new_podcast_config", sub_matches)) => {
            let save_path: &PathBuf = sub_matches
                .get_one("output_path")
                .expect("A directory was provided");
            make_podcast_info_starter(&save_path).expect("Make podcast info config");
            println!("Starter podcast file generated: {}", save_path.display());
        }
        Some(("new_episode", sub_matches)) => {
            let save_dir: &PathBuf = sub_matches
                .get_one("output_directory")
                .expect("A directory was provided");
            let episode_path = save_dir.join("episode.yaml");
            let outline_path = save_dir.join("outline.txt");
            make_episode_starter(&episode_path).expect("Make episode info");
            make_outline_starter(&outline_path).expect("Make outline info");

            println!("Starter episode files generated: {}", save_dir.display())
        }
        Some(("make_markdown", sub_matches)) => {
            let podcast_path: &PathBuf = sub_matches
                .get_one("podcast_path")
                .expect("A podcast file was provided");
            let episode_path: &PathBuf = sub_matches
                .get_one("episode_path")
                .expect("An episode file was provided");
            let outline_path: &PathBuf = sub_matches
                .get_one("outline_path")
                .expect("A time codes file was provided");
            let out_file_path: &PathBuf = sub_matches
                .get_one("out_file_path")
                .expect("An output file was provided");

            generate_content_markdown(podcast_path, episode_path, outline_path, out_file_path)
                .expect("Markdown to generate");

            println!("Markdown generated: {}", out_file_path.display());
        }
        _ => unreachable!("Subcommand should be provided"),
    }
}

fn cmd() -> Command {
    command!()
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .author("Audrow Nash")
        .about("Makes descriptions for podcast episodes")
        .subcommand(
            Command::new("new_podcast_config")
                .about("Make podcast config file")
                .arg(
                    arg!(
                        -o --output_path <path> "Path to the directory where the podcast file will be created"
                    )
                    .default_value("podcast.yaml")
                    .value_parser(value_parser!(PathBuf)),
                )
        )
        .subcommand(
            Command::new("new_episode")
                .about("Makes a starter episode file")
                .arg(
                    arg!(
                        -o --output_directory <dir> "Path to the directory where the episode files will be created"
                    )
                    .default_value(".")
                    .value_parser(value_parser!(PathBuf)),
                )
        )
        .subcommand(
            Command::new("make_markdown")
                .about("Generates a markdown file from description info")
                .arg(
                    arg!(
                        <podcast_path> "Path to the podcast file"
                    )
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(
                        <episode_path> "Path to the episode file"
                    )
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(
                        <outline_path> "Path to the outline file"
                    )
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(
                        -o --out_file_path <file> "Path for where to save the output markdown file"
                    )
                    .default_value("content.md")
                    .value_parser(value_parser!(PathBuf)),
                ),
        )
}

#[test]
fn verify_cmd() {
    cmd().debug_assert();
}
