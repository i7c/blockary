use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

pub fn read_block_strings(markdown_content: &str) -> Vec<String> {
    let parser = Parser::new(markdown_content);

    let mut blocks: Vec<String> = Vec::new();

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
                    blocks.push(item_content.to_string());
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
