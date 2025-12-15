#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    period: String,
    origin: String,
    desc: String,
}

// ... struct Block remains the same ...

impl Block {

    pub fn parse(default_origin: &str, input: &str) -> Result<Self, String> {
        let period_end_index;
        let content_start_index;
        let mut origin = default_origin.to_string();

        if let Some(idx) = input.find('[') {
            period_end_index = idx;
            content_start_index = idx;
        } else {
            let trimmed_period = input.trim_start_matches(|c: char| c.is_ascii_digit() || c == ':' || c == ' ' || c == '-');
            period_end_index = input.len() - trimmed_period.len();
            content_start_index = period_end_index;
        }

        let period = input[..period_end_index].trim().to_string();
        let content = input[content_start_index..].trim_start(); // Content starts here

        let mut description = content;

        if description.starts_with('[') {
            if let Some(end_bracket_index) = description.find(']') {
                // Extract the origin tag (excluding brackets)
                origin = description[1..end_bracket_index].trim().to_string();

                // The description starts after the closing bracket, trimming leading space
                let start_desc_index = end_bracket_index + 1;
                description = description[start_desc_index..].trim_start();
            }
        }

        // Final arbitrary description
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

}
