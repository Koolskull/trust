//                    ☦
//   Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

use crate::app::Key;
use crate::MAX_AGE;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cli {
    Help,
    Run(Launch),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Launch {
    pub age: u8,
    pub dump: bool,
    pub keys: Vec<Key>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsageError {
    pub message: String,
}

pub fn usage_text() -> &'static str {
    "\
KOS TRUST LESSON 3yo

Ages 0 through 14 start that year's lesson.
Age 15 and older opens the full class list. You pick the subject.

  KOS TRUST LESSON 3yo
  KOS TRUST LESSON 15yo
  KOS TRUST LESSON 7yo --dump
  KOS TRUST LESSON 3yo --keys space,2,q
"
}

pub fn parse_cli<S: AsRef<str>>(args: &[S]) -> Result<Cli, UsageError> {
    let args: Vec<String> = args.iter().map(|s| s.as_ref().to_string()).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        return Ok(Cli::Help);
    }

    let mut dump = false;
    let mut keys = Vec::new();
    let mut positional = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--dump" {
            dump = true;
            i += 1;
            continue;
        }
        if arg == "--keys" {
            let spec = args.get(i + 1).ok_or_else(|| err("--keys needs a list"))?;
            keys = parse_keys(spec)?;
            i += 2;
            continue;
        }
        if arg.starts_with("--") {
            return Err(err(format!("unknown flag {arg}")));
        }
        positional.push(arg.clone());
        i += 1;
    }

    if positional.len() != 3 {
        return Err(err(
            "usage: KOS TRUST LESSON 3yo\nexample ages: 3yo, 14yo, 15yo",
        ));
    }
    if !positional[0].eq_ignore_ascii_case("trust") || !positional[1].eq_ignore_ascii_case("lesson")
    {
        return Err(err(
            "usage: KOS TRUST LESSON 3yo\nthe words are TRUST LESSON, then the age",
        ));
    }
    let age = parse_age(&positional[2])?;
    Ok(Cli::Run(Launch { age, dump, keys }))
}

fn parse_age(token: &str) -> Result<u8, UsageError> {
    let lower = token.trim().to_ascii_lowercase();
    let digits = lower
        .strip_suffix("yo")
        .or_else(|| lower.strip_suffix("y"))
        .or_else(|| lower.strip_suffix("yr"))
        .or_else(|| lower.strip_suffix("yrs"))
        .unwrap_or(&lower);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(err(format!(
            "{token} is not an age. Use 3yo, 14yo, or 15yo."
        )));
    }
    let age: u32 = digits.parse().map_err(|_| err("age is not a number"))?;
    if age > u32::from(MAX_AGE) {
        return Err(err(format!("age {age} is past {MAX_AGE}")));
    }
    Ok(age as u8)
}

fn parse_keys(spec: &str) -> Result<Vec<Key>, UsageError> {
    let mut keys = Vec::new();
    for part in spec.split(',') {
        let name = part.trim().to_ascii_lowercase();
        if name.is_empty() {
            continue;
        }
        let key = match name.as_str() {
            "space" | "button" => Key::Char(' '),
            "esc" | "escape" | "quit" => Key::Quit,
            "q" => Key::Char('q'),
            "b" | "back" => Key::Char('b'),
            other if other.chars().count() == 1 => Key::Char(other.chars().next().unwrap_or('?')),
            other => {
                return Err(err(format!(
                    "unknown key {other}. Use space, q, b, or a single character."
                )));
            }
        };
        keys.push(key);
    }
    if keys.is_empty() {
        return Err(err("--keys needs at least one key"));
    }
    Ok(keys)
}

fn err(message: impl Into<String>) -> UsageError {
    UsageError {
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_yo_is_the_guided_launch() {
        let Cli::Run(launch) = parse_cli(&["TRUST", "LESSON", "3yo"]).unwrap() else {
            panic!("help");
        };
        assert_eq!(launch.age, 3);
        assert!(launch.keys.is_empty());
    }

    #[test]
    fn command_is_case_insensitive_and_accepts_plain_years() {
        let Cli::Run(launch) = parse_cli(&["trust", "lesson", "15YO"]).unwrap() else {
            panic!("help");
        };
        assert_eq!(launch.age, 15);
        let Cli::Run(plain) = parse_cli(&["Trust", "Lesson", "7"]).unwrap() else {
            panic!("help");
        };
        assert_eq!(plain.age, 7);
    }

    #[test]
    fn keys_and_dump_flags() {
        let Cli::Run(launch) =
            parse_cli(&["TRUST", "LESSON", "3yo", "--keys", "space,2,q"]).unwrap()
        else {
            panic!("help");
        };
        assert_eq!(
            launch.keys,
            vec![Key::Char(' '), Key::Char('2'), Key::Char('q')]
        );
        let Cli::Run(dump) = parse_cli(&["TRUST", "LESSON", "--dump", "14y"]).unwrap() else {
            panic!("help");
        };
        assert!(dump.dump);
        assert_eq!(dump.age, 14);
    }

    #[test]
    fn rejects_a_missing_age_and_a_huge_age() {
        assert!(parse_cli(&["TRUST", "LESSON"]).is_err());
        assert!(parse_cli(&["LESSON", "3yo"]).is_err());
        assert!(parse_cli(&["TRUST", "LESSON", "121yo"]).is_err());
        assert!(matches!(parse_cli(&["--help"]), Ok(Cli::Help)));
    }
}
