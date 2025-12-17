use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use crate::block::{Block};

#[derive(Debug)]
pub struct DayPlan {
    pub origin: String,
    pub blocks: Vec<Block>,
}

impl DayPlan {
    pub fn from_markdown(markdown_content: &str, origin: &str) -> DayPlan {
        let blocks = read_blocks_from_markdown(markdown_content, origin);
        DayPlan { origin: origin.to_string(), blocks: blocks }
    }

    pub fn only_original_blocks(self: DayPlan) -> DayPlan {
        let orig_blocks = self.blocks.into_iter().filter(|b| b.origin == self.origin).collect();
        DayPlan {
            origin: self.origin,
            blocks: orig_blocks,
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
            Event::Start(Tag::Item{}) => {
                if in_block {
                    item_content.clear();
                    in_item = true;
                }
            },
            Event::End(TagEnd::Item) => {
                if in_item {
                    blocks.push(Block::parse(origin, &item_content).expect(""));
                    in_item = false;
                }
            },
        
            Event::Start(Tag::Heading{ level: HeadingLevel::H2, id: _, classes: _, attrs: _} ) => {
                check_block = true;
                in_block = false;
            }
            Event::End(TagEnd::Heading(HeadingLevel::H2)) => check_block = false,

        
            Event::Text(text) => {
                if check_block && text.to_lowercase().trim() == "time blocks" {
                    check_block = false;
                    in_block = true;
                }
                if in_item {
                    item_content.push_str(&text);
                }
            },

            _ => {},
        }
    }

    blocks
}
