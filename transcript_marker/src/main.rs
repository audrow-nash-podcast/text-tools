use clap::{arg, command, value_parser};
use std::path::PathBuf;

mod mark_transcript;
use mark_transcript::mark_transcript;

use common::parse_outline;

fn main() {
    let matches = command!()
        .version("1.0")
        .author("Audrow Nash")
        .about("Marks a transcript with time codes and adds a table of contents")
        .arg(
            arg!(
                <transcript_path> "Sets the path to the transcript file"
            )
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                <outline_path> "Sets the path to the outline file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --out_file_path <file> "Path for where to save the output file"
            )
            .default_value("marked_transcript.md")
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let transcript_path: &PathBuf = matches
        .get_one("transcript_path")
        .expect("A transcript file was provided");
    let outline_path: &PathBuf = matches
        .get_one("outline_path")
        .expect("A time codes file was provided");
    let out_file_path: &PathBuf = matches
        .get_one("out_file_path")
        .expect("An output file was provided");

    let transcript =
        std::fs::read_to_string(transcript_path).expect("The transcript file to be read");
    let outline = std::fs::read_to_string(outline_path).expect("The time codes file to be read");

    let mut outline_entries = parse_outline(&outline).expect("The time codes file to be parsed");
    let new_transcript =
        mark_transcript(&transcript, &mut outline_entries).expect("The transcript to be marked");

    std::fs::write(out_file_path, new_transcript).expect("The output file to be written");
}
