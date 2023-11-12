use std::path::PathBuf;
use clap::{arg, command, value_parser, ArgAction, Command};

mod mark_transcript;
use mark_transcript::mark_transcript;

mod parse_outline;
use parse_outline::parse_outline;

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(
                -t --transcript <FILE> "Sets the path to the transcript file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -c --time_codes <FILE> "Sets the path to the time codes file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --out_file <FILE> "Optional path for where to save the output file"
            )
            .default_value("marked_transcript.txt")
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let transcript_path: &PathBuf = matches.get_one("transcript").expect("transcript is required");
    let time_codes_path: &PathBuf = matches.get_one("time_codes").expect("time_codes is required");
    let out_path: &PathBuf = matches.get_one("out_file").expect("out_file is required");

    println!("transcript: {:?}", transcript_path);
    println!("time_codes: {:?}", time_codes_path);
    println!("out_file: {:?}", out_path);

    let transcript = std::fs::read_to_string(transcript_path).expect("failed to read transcript file");
    let time_codes = std::fs::read_to_string(time_codes_path).expect("failed to read time codes file");

    let mut outline_entries = parse_outline(&time_codes).expect("failed to parse outline");
    let new_transcript = mark_transcript(&transcript, &mut outline_entries).unwrap();

    std::fs::write(out_path, new_transcript).expect("failed to write output file");

}
