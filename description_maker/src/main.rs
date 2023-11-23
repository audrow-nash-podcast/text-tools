use clap::{arg, command, value_parser, Command};
use std::path::PathBuf;

mod template;
use crate::template::{generate_content_markdown, make_episode_starter, make_podcast_info_starter};

mod types;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .author("Audrow Nash")
        .about("Makes descriptions for podcast episodes")
        .subcommand(
            Command::new("init")
                .about("Makes starter files")
                .arg(
                    arg!(
                        -o --out_dir <dir> "Path for where to save the output files"
                    )
                    .default_value("out")
                    .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(
                        --episode_file_name <file> "Name of the episode file"
                    )
                    .default_value("episode.yaml")
                    .value_parser(value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("new_episode")
                .about("Makes a starter episode file")
                .arg(
                    arg!(
                        <episode_file_path> "Path for where to put the episode file"
                    )
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(
                        --episode_file_name <file> "Name of the episode file"
                    )
                    .default_value("episode.yaml")
                    .value_parser(value_parser!(String)),
                ),
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
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let out_dir: &String = sub_matches.get_one("out_dir").expect("Out dir has a value");
            let episode_file_name: &String = sub_matches
                .get_one("episode_file_name")
                .expect("Episode file name has a value");

            make_podcast_info_starter(out_dir).expect("Make podcast info");
            make_episode_starter(out_dir, episode_file_name).expect("Make episode info");

            println!("Starter files generated in: {}", out_dir)
        }
        Some(("new_episode", sub_matches)) => {
            let episode_file_path: &PathBuf = sub_matches
                .get_one("episode_file_path")
                .expect("An episode file was provided");

            let save_dir = episode_file_path
                .parent()
                .expect("The episode file has a parent directory")
                .to_str()
                .expect("The episode file is converted to a string");
            let episode_file_name = episode_file_path
                .file_name()
                .expect("The episode file has a file name")
                .to_str()
                .expect("The episode file name is converted to a string");

            make_episode_starter(save_dir, episode_file_name).expect("Make episode info");

            println!(
                "Starter episode file generated: {}",
                episode_file_path.display()
            )
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
