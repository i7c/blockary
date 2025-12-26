use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

pub fn line_is_heading(line: &str, heading: &str) -> bool {
    if line.trim().starts_with("#") {
        line.trim_start_matches(['#', ' ']) == heading
    } else {
        false
    }
}

pub fn line_is_any_heading(line: &str) -> bool {
    return line.trim().starts_with("#");
}

/// Updates the content in `markdown_content`'s first markdown section
/// with title `section_title`. Returns the updated markdown as a
/// string. If no such section can be found, nothing is changed in and
/// the returned string is identical with markdown_content.
pub fn update_section_lines(
    section_lines: &Vec<String>,
    section_title: &str,
    markdown_content: &str,
) -> String {
    let mut output_lines: Vec<String> = Vec::new();

    let mut under_heading = false;
    for l in markdown_content.lines() {
        if line_is_heading(l, section_title) {
            under_heading = true;
            output_lines.push(l.to_string());
            output_lines.extend(
                section_lines
                    .iter()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>(),
            );
            continue;
        }

        if line_is_any_heading(l) {
            under_heading = false;
        }

        if under_heading {
            continue;
        }
        output_lines.push(l.to_string());
    }

    output_lines.push("".to_string());
    output_lines.join("\n")
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_with_faulty_elements() {
        let markdown = "
# Some Title
## Some other section
```clojure
(+ 1 2)
```

```tasks
some other suprising content %#$@
like $(exit 0)
```
bla foo
   ### Some wrong indent
## Time Blocks
- 08:00 - 11:00 This
- 11:00 - 11:30 should
- 14:00 - 15:00 appear
- So should this
# Notes
- 10:00 - 11:00 This should not appear in the result
";
        let block_strings = read_block_strings(markdown);

        assert_eq!(
            block_strings,
            vec![
                "08:00 - 11:00 This".to_string(),
                "11:00 - 11:30 should".to_string(),
                "14:00 - 15:00 appear".to_string(),
                "So should this".to_string(),
            ],
        );
    }

    #[test]
    fn test_update_blockstrings_in_markdown_with_faulty_elements() {
        let markdown = "
# Some Title
## Some other section
```clojure
(+ 1 2)
```

```tasks
some other suprising content %#$@
like $(exit 0)
```
bla foo
   ### Some wrong indent
## Time Blocks
- 08:00 - 11:00 This
- 11:00 - 11:30 should
- 14:00 - 15:00 appear
- So should this
# Notes
- 10:00 - 11:00 What is this?
";
        let block_strings = vec![
            "- 10:00 - 11:00 (Personal) -hidden-".to_string(),
            "- 11:00 - 12:00 Meeting".to_string(),
        ];

        let updated_markdown = update_section_lines(&block_strings, "Time Blocks", markdown);

        assert_eq!(
            updated_markdown,
            "
# Some Title
## Some other section
```clojure
(+ 1 2)
```

```tasks
some other suprising content %#$@
like $(exit 0)
```
bla foo
   ### Some wrong indent
## Time Blocks
- 10:00 - 11:00 (Personal) -hidden-
- 11:00 - 12:00 Meeting
# Notes
- 10:00 - 11:00 What is this?
"
        );
    }

    #[test]
    fn test_check_for_heading() {
        assert_eq!(true, line_is_heading("### Foo Bar", "Foo Bar"));
        assert_eq!(true, line_is_heading(" # Foo Bar", "Foo Bar"));
    }

    #[test]
    fn test_line_is_not_heading() {
        assert_eq!(false, line_is_heading("Foo Bar", "Foo Bar"));
    }
}
