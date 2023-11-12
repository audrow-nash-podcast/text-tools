use std::error::Error;
use regex::Regex;
use crate::types::{TimeCode, OutlineEntry};

pub fn mark_transcript(transcript: &str) -> Result<(), Box<dyn Error>> {

    let line_regex = Regex::new(r"\[(\d{2,}:\d{2}:\d{2})\] .*").expect("Line regex is valid");

    let mut outline_entries: Vec<OutlineEntry> = vec![
        OutlineEntry {
            time_code: TimeCode {
                hours: 0,
                minutes: 0,
                seconds: 0,
            },
            text: "Introduction".into(),
        },
        OutlineEntry {
            time_code: TimeCode {
                hours: 0,
                minutes: 3,
                seconds: 09,
            },
            text: "Nag and Mike introduce themselves".into(),
        },
    ];
    outline_entries.sort();

    let mut out: Vec<String> = vec![];
    for line in transcript.lines() {
        if let Some(captures) = line_regex.captures(line) {
            let time_code = TimeCode::from_str(&captures[1])?;
            if time_code >= outline_entries[0].time_code {
                let entry = outline_entries.remove(0);
                out.push(format!("## {}\n", entry.text));
            }
        }
        out.push(format!("{}", line.trim()));
    }

    let out = out.join("\n");
    println!("{}", out);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::mark_transcript;

    #[test]
    fn put_text_before_timecodes() {
        let transcript = r#"[00:00:00] **Audrow Nash:** I've talked to a lot of people about this interview, and I'm excited that I get to share it with you. In it, I talk with Nag and Mike from Electric Sheep. They're doing many things differently than most robotics companies that I've talked to, and they're making big bets that I think will pay off. Here are three examples to show you what I mean.

First, they're throwing away classical robotics approaches, and instead... using machine learning. I'm not just talking about for perception or parameter optimization, but even for things like localization or high level control. Second, they've turned the lawn mowing problem on its head to make robots that are intrinsically safer.

And third, instead of selling their robots or doing a subscription model, they buy profitable landscaping businesses and give those companies robots. There are a lot of advantages to this last point and you'll see the details of each during the interview. I hope it surprises you as much as it did me. I think you'll enjoy this interview if you're curious about how AI and robotics can fit together in a real application and if you want to see a new robotics business model that I think will be very popular in the near future.

[00:01:37] **Nag Murty:** Yeah. Hi everyone. I'm Nag Murthy. I'm the CEO and co founder of Electric Sheep Robotics. 

And then over time, we plan to use those, automated robots to improve our own margins over time. So that's what we do. 

[00:03:09] **Michael Laskey:** Yeah, I can take that one."#;

        mark_transcript(transcript).expect("Mark transcript should succeed");

    }

}
