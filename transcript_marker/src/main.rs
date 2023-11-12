use clap::{arg, command, value_parser};
use std::path::PathBuf;

mod mark_transcript;
use mark_transcript::mark_transcript;

mod parse_outline;
use parse_outline::parse_outline;

mod types;

fn main() {
    let matches = command!()
        .version("1.0")
        .author("Audrow Nash")
        .about("Marks a transcript with time codes and adds a table of contents")
        .arg(
            arg!(
                <transcript> "Sets the path to the transcript file"
            )
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                <time_codes> "Sets the path to the time codes file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --out_file <file> "Path for where to save the output file"
            )
            .default_value("marked_transcript.txt")
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let transcript_path: &PathBuf = matches
        .get_one("transcript")
        .expect("A transcript file was provided");
    let time_codes_path: &PathBuf = matches
        .get_one("time_codes")
        .expect("A time codes file was provided");
    let out_path: &PathBuf = matches
        .get_one("out_file")
        .expect("An output file was provided");

    let transcript =
        std::fs::read_to_string(transcript_path).expect("The transcript file to be read");
    let time_codes =
        std::fs::read_to_string(time_codes_path).expect("The time codes file to be read");

    let mut outline_entries = parse_outline(&time_codes).expect("The time codes file to be parsed");
    let new_transcript =
        mark_transcript(&transcript, &mut outline_entries).expect("The transcript to be marked");

    std::fs::write(out_path, new_transcript).expect("The output file to be written");
}
