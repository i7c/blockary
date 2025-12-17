use crate::block::Block;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

#[derive(Debug)]
pub struct DayPlan {
    pub origin: String,
    pub blocks: Vec<Block>,
}

impl DayPlan {
    pub fn from_markdown(markdown_content: &str, origin: &str) -> DayPlan {
        let blocks = read_blocks_from_markdown(markdown_content, origin);
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

    pub fn merge(self: DayPlan, other: DayPlan) -> DayPlan {
        let mut blocks = self.blocks;
        blocks.extend(other.blocks);
        blocks.sort_by(|a, b| a.period.cmp(&b.period));
        DayPlan {
            origin: self.origin,
            blocks: blocks,
        }
    }
}

pub fn read_blocks_from_markdown(markdown_content: &str, origin: &str) -> Vec<Block> {
    let parser = Parser::new(markdown_content);

    let mut blocks: Vec<Block> = Vec::new();

    let mut check_block = false;
    let mut in_block = false;
    let mut in_item = false;
    let mut item_content = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Item {}) => {
                if in_block {
                    item_content.clear();
                    in_item = true;
                }
            }
            Event::End(TagEnd::Item) => {
                if in_item {
                    blocks.push(Block::parse(origin, &item_content).expect(""));
                    in_item = false;
                }
            }

            Event::Start(Tag::Heading {
                level: _,
                id: _,
                classes: _,
                attrs: _,
            }) => {
                check_block = true;
                in_block = false;
            }
            Event::End(TagEnd::Heading(HeadingLevel::H2)) => check_block = false,

            Event::Text(text) => {
                if check_block {
                    check_block = false;
                    in_block = text.to_lowercase().trim() == "time blocks";
                }
                if in_item {
                    item_content.push_str(&text);
                }
            }
            _ => {}
        }
    }

    blocks
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
