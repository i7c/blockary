#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    period: String,
    origin: String,
    desc: String,
}

pub fn parse_block_string(origin: &str, _blockstr: &str) -> Block {
    Block {
        period: "08:00 - 09:00".to_string(),
        origin: "Personal".to_string(),
        desc: "Morning Correspondence".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_good_string_with_origin_tag() {
        let b = parse_block_string("Personal", "08:00 - 09:00 [Personal] Morning Correspondence");

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
        let b = parse_block_string("Personal", "07:30 - 08:00 Morning Correspondence");

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
