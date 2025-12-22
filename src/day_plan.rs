use crate::block::Block;
use crate::markdown_access;

#[derive(Debug, Clone)]
pub struct DayPlan {
    pub origin: String,
    pub blocks: Vec<Block>,
}

impl DayPlan {
    pub fn from_markdown(markdown_content: &str, origin: &str) -> DayPlan {
        let block_strings = markdown_access::read_block_strings(markdown_content);
        let blocks = block_strings
            .iter()
            .map(|bs| Block::parse(origin, bs).expect(""))
            .collect();

        DayPlan {
            origin: origin.to_string(),
            blocks: blocks,
        }
    }

    pub fn only_original_blocks(self: DayPlan) -> DayPlan {
        let orig_blocks = self
            .blocks
            .into_iter()
            .filter(|b| b.origin == self.origin)
            .collect();
        DayPlan {
            origin: self.origin,
            blocks: orig_blocks,
        }
    }

    pub fn merge(self: &DayPlan, other: &DayPlan) -> DayPlan {
        let mut blocks = self.blocks.clone();
        blocks.extend(other.blocks.clone());
        blocks.sort_by(|a, b| a.period.cmp(&b.period));
        DayPlan {
            origin: self.origin.clone(),
            blocks: blocks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_blocks() {
        let markdown = "
# Some Title
## Some other section
bla foo
## Time Blocks
- 08:00 - 11:00 This
- 11:00 - 11:30 should
- 14:00 - 15:00 appear
- So should this
# Notes
- 10:00 - 11:00 This should not appear in the result
";
        let plan = DayPlan::from_markdown(&markdown, "Personal");

        assert_eq!(
            plan.blocks,
            vec![
                Block {
                    period: "08:00 - 11:00".to_string(),
                    origin: "Personal".to_string(),
                    desc: "This".to_string()
                },
                Block {
                    period: "11:00 - 11:30".to_string(),
                    origin: "Personal".to_string(),
                    desc: "should".to_string()
                },
                Block {
                    period: "14:00 - 15:00".to_string(),
                    origin: "Personal".to_string(),
                    desc: "appear".to_string()
                },
                Block {
                    period: "".to_string(),
                    origin: "Personal".to_string(),
                    desc: "So should this".to_string()
                },
            ],
        );
    }

    #[test]
    fn test_merge_two_plans() {
        let plan1 = DayPlan {
            origin: "Personal".to_string(),
            blocks: vec![
                Block {
                    period: "08:00 - 11:00".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Morning Coffee".to_string(),
                },
                Block {
                    period: "14:00 - 14:30".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Walk".to_string(),
                },
            ],
        };

        let plan2 = DayPlan {
            origin: "Work".to_string(),
            blocks: vec![
                Block {
                    period: "09:00 - 09:30".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Morning Brief".to_string(),
                },
                Block {
                    period: "12:00 - 12:30".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Lunch".to_string(),
                },
            ],
        };

        let merged = plan1.merge(plan2);

        assert_eq!(merged.origin, "Personal");
        assert_eq!(
            merged.blocks,
            vec![
                Block {
                    period: "08:00 - 11:00".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Morning Coffee".to_string(),
                },
                Block {
                    period: "09:00 - 09:30".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Morning Brief".to_string(),
                },
                Block {
                    period: "12:00 - 12:30".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Lunch".to_string(),
                },
                Block {
                    period: "14:00 - 14:30".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Walk".to_string(),
                },
            ]
        );
    }
}
