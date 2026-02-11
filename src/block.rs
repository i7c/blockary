use regex::Regex;

use crate::tag::{Tag, parse_tags};

const BLOCKSTRING_REGEX: &str =
    r"^\s*(\d{2}:\d{2}\s*-\s*\d{2}:\d{2}|\d{2}:\d{2})?\s*(\(([^\)]*)\))?\s*(.*)";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub period_str: String,
    pub origin: String,
    pub desc: String,
    pub duration: u16,
    pub tags: Vec<Tag>,
}

impl Block {
    pub fn new(period_str: &str, origin: &str, desc: &str) -> Self {
        Block {
            period_str: period_str.to_string(),
            origin: origin.to_string(),
            desc: desc.to_string(),
            duration: duration_of_period(period_str).unwrap_or_else(|| 30),
            tags: parse_tags(desc),
        }
    }

    pub fn parse_block_string(default_origin: &str, input: &str) -> Result<Self, String> {
        match Regex::new(BLOCKSTRING_REGEX).unwrap().captures(&input) {
            Some(matches) => {
                let period = matches.get(1).map(|m| m.as_str().to_string());
                let origin = matches.get(3).map(|m| m.as_str().to_string());
                let desc = matches.get(4).map(|m| m.as_str().to_string());

                if let Some(desc) = desc {
                    return Ok(Block::new(
                        &period.unwrap_or("".to_string()),
                        &origin.unwrap_or(default_origin.to_string()),
                        &desc,
                    ));
                } else {
                    return Err("Blockstring must have at least a description".to_string());
                }
            }
            None => return Err("Not a valid block string".to_string()),
        }
    }

    pub fn to_block_string(self: &Block, include_origin: bool) -> String {
        if include_origin {
            format!("{} ({}) {}", self.period_str, self.origin, self.desc)
        } else {
            format!("{} {}", self.period_str, self.desc)
        }
    }
}

fn duration_of_period(period: &str) -> Option<u16> {
    let parts: Vec<&str> = period.split("-").map(|s| s.trim()).collect();

    if parts.len() != 2 {
        return None;
    }

    let start_minutes = parse_to_minutes(parts[0])?;
    let end_minutes = parse_to_minutes(parts[1])?;

    // Since times are always on the same day, end should be >= start
    if end_minutes >= start_minutes {
        Some(end_minutes - start_minutes)
    } else {
        None
    }
}

fn parse_to_minutes(time_str: &str) -> Option<u16> {
    let mut parts = time_str.split(':');
    let hours: u32 = parts.next()?.parse().ok()?;
    let minutes: u32 = parts.next()?.parse().ok()?;

    if hours < 24 && minutes < 60 {
        Some((hours * 60 + minutes).try_into().unwrap())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_good_string_with_origin_tag() {
        let b = Block::parse_block_string(
            "Personal",
            "08:00 - 09:00 (Personal) Morning Correspondence",
        )
        .expect("");

        assert_eq!(
            b,
            Block {
                period_str: "08:00 - 09:00".to_string(),
                origin: "Personal".to_string(),
                desc: "Morning Correspondence".to_string(),
                duration: 60,
                tags: vec![],
            }
        );
    }

    #[test]
    fn test_parse_good_string_without_origin_tag() {
        let b = Block::parse_block_string("Personal", "07:30 - 08:00 Morning Correspondence")
            .expect("");

        assert_eq!(
            b,
            Block {
                period_str: "07:30 - 08:00".to_string(),
                origin: "Personal".to_string(),
                desc: "Morning Correspondence".to_string(),
                duration: 30,
                tags: vec![],
            }
        );
    }

    #[test]
    fn test_parse_good_string_without_origin_tag_but_brackets() {
        let b = Block::parse_block_string(
            "Personal",
            "07:30 - 08:00 Morning Correspondence: talk to [[Lars]] later",
        )
        .expect("");

        assert_eq!(
            b,
            Block {
                period_str: "07:30 - 08:00".to_string(),
                origin: "Personal".to_string(),
                desc: "Morning Correspondence: talk to [[Lars]] later".to_string(),
                duration: 30,
                tags: vec![],
            }
        );
    }

    #[test]
    fn test_parse_good_string_that_starts_with_digit() {
        let b = Block::parse_block_string("Personal", "07:30 - 08:00 1on1 with Hans").expect("");

        assert_eq!(
            b,
            Block {
                period_str: "07:30 - 08:00".to_string(),
                origin: "Personal".to_string(),
                desc: "1on1 with Hans".to_string(),
                duration: 30,
                tags: vec![],
            }
        );
    }

    #[test]
    fn test_block_without_period_or_origin() {
        let b = Block::parse_block_string("Personal", "Just some text").expect("");

        assert_eq!(
            b,
            Block {
                period_str: "".to_string(),
                origin: "Personal".to_string(),
                desc: "Just some text".to_string(),
                duration: 30,
                tags: vec![],
            }
        );
    }

    #[test]
    fn test_block_with_empty_description() {
        let b = Block::parse_block_string("Personal", "10:00 - 11:00").expect("");

        assert_eq!(
            b,
            Block {
                period_str: "10:00 - 11:00".to_string(),
                origin: "Personal".to_string(),
                desc: "".to_string(),
                duration: 60,
                tags: vec![],
            }
        );
    }

    #[test]
    fn test_period_has_only_start_time() {
        let b = Block::parse_block_string("Personal", "10:00 Do something").expect("");

        assert_eq!(
            b,
            Block {
                period_str: "10:00".to_string(),
                origin: "Personal".to_string(),
                desc: "Do something".to_string(),
                duration: 30,
                tags: vec![],
            }
        );
    }

    #[test]
    fn test_parse_block_string_with_period_in_desc() {
        let b =
            Block::parse_block_string("Personal", "A desc with random period from 10:00 - 11:00")
                .expect("");

        assert_eq!(
            b,
            Block {
                period_str: "".to_string(),
                origin: "Personal".to_string(),
                desc: "A desc with random period from 10:00 - 11:00".to_string(),
                duration: 30,
                tags: vec![],
            }
        );
    }

    #[test]
    fn test_parse_duplicate_period() {
        let b = Block::parse_block_string("Personal", "10:00 - 11:00 10:00 - 11:00").expect("");

        assert_eq!(
            b,
            Block {
                period_str: "10:00 - 11:00".to_string(),
                origin: "Personal".to_string(),
                desc: "10:00 - 11:00".to_string(),
                duration: 60,
                tags: vec![],
            }
        );
    }

    #[test]
    fn test_write_block_string() {
        let b = Block {
            period_str: "10:00 - 11:00".to_string(),
            origin: "Personal".to_string(),
            desc: "Buy Coffee".to_string(),
            duration: 60,
            tags: vec![],
        };

        assert_eq!(
            b.to_block_string(true),
            "10:00 - 11:00 (Personal) Buy Coffee"
        );
        assert_eq!(b.to_block_string(false), "10:00 - 11:00 Buy Coffee");
    }

    #[test]
    fn test_maximum_duration() {
        let b = Block::new("00:00 - 23:59", "banana", "asdf");
        assert_eq!(b.duration, 1439);
    }

    #[test]
    fn test_tags_are_parsed_and_added() {
        let b =
            Block::parse_block_string("none", "10:00 - 11:00 Buy coffee @chores @personal/tasks")
                .unwrap();

        assert_eq!(b.tags.get(0).unwrap().tagls, vec!["chores"]);
        assert_eq!(b.tags.get(1).unwrap().tagls, vec!["personal", "tasks"]);
    }
}
