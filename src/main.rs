use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use std::fs;

mod block;

fn main() {
    let content = fs::read_to_string("/Users/cmw/git/criptonotes/brain/journal/2025/2025-12-11.md")
        .expect("should have read the file");
    let parser = Parser::new(&content);

    let mut blocks: Vec<block::Block> = Vec::new();

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
                    blocks.push(block::parse_block_string("Personal", &item_content));
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
                    println!("found: {text}");
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
    
}
