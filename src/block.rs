#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    period: String,
    origin: String,
    desc: String,
}

// ... struct Block remains the same ...

impl Block {

    pub fn parse(default_origin: &str, input: &str) -> Result<Self, String> {
        let period_end;
        let desc_start;
        let mut origin = default_origin.to_string();

        if let Some(idx) = input.find('[') {
            period_end = idx;
            desc_start = idx;
        } else {
            let trimmed_period = input.trim_start_matches(
                |c: char| c.is_ascii_digit() || c == ':' || c == ' ' || c == '-'
            );
            period_end = input.len() - trimmed_period.len();
            desc_start = period_end;
        }

        let period = input[..period_end].trim().to_string();

        let mut description = input[desc_start..].trim_start();
        if description.starts_with('[') {
            if let Some(end_bracket_index) = description.find(']') {
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

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_good_string_with_origin_tag() {
        let b = Block::parse("Personal", "08:00 - 09:00 [Personal] Morning Correspondence").expect("");

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
        let b = Block::parse("Personal", "07:30 - 08:00 Morning Correspondence: talk to [[Lars]] later").expect("");

        assert_eq!(
            b,
            Block {
                period: "07:30 - 08:00".to_string(),
                origin: "Personal".to_string(),
                desc: "Morning Correspondence: talk to [[Lars]] later".to_string(),
            }
        );
    }


}
