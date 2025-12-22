#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub period: String,
    pub origin: String,
    pub desc: String,
}

impl Block {
    pub fn parse(default_origin: &str, input: &str) -> Result<Self, String> {
        let trimmed_period = input
            .trim_start_matches(|c: char| c.is_ascii_digit() || c == ':' || c == ' ' || c == '-');
        let period_end = input.len() - trimmed_period.len();
        let desc_start = period_end;

        let period = input[..period_end].trim().to_string();

        let mut description = input[desc_start..].trim_start();
        let mut origin = default_origin.to_string();
        if description.starts_with('(') {
            if let Some(end_bracket_index) = description.find(')') {
                origin = description[1..end_bracket_index].trim().to_string();
                description = description[end_bracket_index + 1..].trim_start();
            }
        }
        let desc = description.to_string();

        Ok(Block {
            period,
            origin,
            desc,
        })
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
