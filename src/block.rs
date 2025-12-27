use regex::Regex;

const BLOCKSTRING_REGEX: &str =
    r"^\s*(\d{2}:\d{2}\s*-\s*\d{2}:\d{2}|\d{2}:\d{2})?\s*(\(([^\)]*)\))?\s*(.*)";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub period: String,
    pub origin: String,
    pub desc: String,
}

impl Block {
    pub fn parse(default_origin: &str, input: &str) -> Result<Self, String> {
        match Regex::new(BLOCKSTRING_REGEX).unwrap().captures(&input) {
            Some(matches) => {
                let period = matches.get(1).map(|m| m.as_str().to_string());
                let origin = matches.get(3).map(|m| m.as_str().to_string());
                let desc = matches.get(4).map(|m| m.as_str().to_string());

                if let Some(desc) = desc {
                    return Ok(Block {
                        period: period.unwrap_or("".to_string()),
                        origin: origin.unwrap_or(default_origin.to_string()),
                        desc,
                    });
                } else {
                    return Err("Blockstring must have at least a description".to_string());
                }
            }
            None => return Err("Not a valid block string".to_string()),
        }
    }

    pub fn to_block_string(self: &Block, include_origin: bool) -> String {
        if include_origin {
            format!("- {} ({}) {}", self.period, self.origin, self.desc)
        } else {
            format!("- {} {}", self.period, self.desc)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_good_string_with_origin_tag() {
        let b = Block::parse(
            "Personal",
            "08:00 - 09:00 (Personal) Morning Correspondence",
        )
        .expect("");

        assert_eq!(
            b,
            Block {
                period: "08:00 - 09:00".to_string(),
                origin: "Personal".to_string(),
                desc: "Morning Correspondence".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_good_string_without_origin_tag() {
        let b = Block::parse("Personal", "07:30 - 08:00 Morning Correspondence").expect("");

        assert_eq!(
            b,
            Block {
                period: "07:30 - 08:00".to_string(),
                origin: "Personal".to_string(),
                desc: "Morning Correspondence".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_good_string_without_origin_tag_but_brackets() {
        let b = Block::parse(
            "Personal",
            "07:30 - 08:00 Morning Correspondence: talk to [[Lars]] later",
        )
        .expect("");

        assert_eq!(
            b,
            Block {
                period: "07:30 - 08:00".to_string(),
                origin: "Personal".to_string(),
                desc: "Morning Correspondence: talk to [[Lars]] later".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_good_string_that_starts_with_digit() {
        let b = Block::parse("Personal", "07:30 - 08:00 1on1 with Hans").expect("");

        assert_eq!(
            b,
            Block {
                period: "07:30 - 08:00".to_string(),
                origin: "Personal".to_string(),
                desc: "1on1 with Hans".to_string(),
            }
        );
    }

    #[test]
    fn test_block_without_period_or_origin() {
        let b = Block::parse("Personal", "Just some text").expect("");

        assert_eq!(
            b,
            Block {
                period: "".to_string(),
                origin: "Personal".to_string(),
                desc: "Just some text".to_string(),
            }
        );
    }

    #[test]
    fn test_block_with_empty_description() {
        let b = Block::parse("Personal", "10:00 - 11:00").expect("");

        assert_eq!(
            b,
            Block {
                period: "10:00 - 11:00".to_string(),
                origin: "Personal".to_string(),
                desc: "".to_string(),
            }
        );
    }

    #[test]
    fn test_period_has_only_start_time() {
        let b = Block::parse("Personal", "10:00 Do something").expect("");

        assert_eq!(
            b,
            Block {
                period: "10:00".to_string(),
                origin: "Personal".to_string(),
                desc: "Do something".to_string(),
            }
        );
    }

    #[test]
    fn test_write_block_string() {
        let b = Block {
            period: "10:00 - 11:00".to_string(),
            origin: "Personal".to_string(),
            desc: "Buy Coffee".to_string(),
        };

        assert_eq!(
            b.to_block_string(true),
            "- 10:00 - 11:00 (Personal) Buy Coffee"
        );
        assert_eq!(b.to_block_string(false), "- 10:00 - 11:00 Buy Coffee");
    }
}
