#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tag {
    pub tagls: Vec<String>,
}

pub fn parse_tags(input: &str) -> Vec<Tag> {
    let mut tags = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((_, c)) = chars.next() {
        if c == '@' {
            // Check if there's at least one char after '+' and it's not whitespace
            if let Some(&(_, next_c)) = chars.peek() {
                if !next_c.is_whitespace() {
                    if let Some(tag) = parse_single_tag(&mut chars) {
                        tags.push(tag);
                    }
                }
            }
        }
    }
    tags
}

fn parse_single_tag(chars: &mut std::iter::Peekable<std::str::CharIndices>) -> Option<Tag> {
    let mut levels = Vec::new();

    loop {
        let mut current_level = String::new();

        match chars.peek() {
            // Case 1: Double Brackets [[...]]
            Some(&(_start_idx, '[')) => {
                let mut bracket_count = 0;
                while let Some((_, c)) = chars.next() {
                    current_level.push(c);
                    if c == '[' {
                        bracket_count += 1;
                    }
                    if c == ']' {
                        bracket_count -= 1;
                    }
                    // Stop once we've closed both brackets
                    if bracket_count == 0 && current_level.ends_with("]]") {
                        break;
                    }
                }
            }
            // Case 2: Parentheses (...)
            Some(&(_start_idx, '(')) => {
                while let Some((_, c)) = chars.next() {
                    current_level.push(c);
                    if c == ')' {
                        break;
                    }
                }
            }
            // Case 3: Standard unquoted level
            _ => {
                while let Some(&(_, c)) = chars.peek() {
                    if c == '/' || c.is_whitespace() || c == '@' {
                        break;
                    }
                    current_level.push(chars.next().unwrap().1);
                }
            }
        }

        if !current_level.is_empty() {
            levels.push(current_level);
        }

        // Check if there is a next level segment
        if let Some(&(_, '/')) = chars.peek() {
            chars.next(); // consume '/'
            continue;
        } else {
            break;
        }
    }

    if levels.is_empty() {
        None
    } else {
        Some(Tag { tagls: levels })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brackets_included() {
        let input = "@p/[[PROJ Vancouver]]";
        let result = parse_tags(input);
        assert_eq!(
            result[0].tagls,
            vec!["p".to_string(), "[[PROJ Vancouver]]".to_string()]
        );
    }

    #[test]
    fn test_parenthesis_included() {
        let input = "@p/(Hi World)/Derp";
        let result = parse_tags(input);
        assert_eq!(
            result[0].tagls,
            vec![
                "p".to_string(),
                "(Hi World)".to_string(),
                "Derp".to_string()
            ]
        );
    }

    #[test]
    fn test_complex_mixed() {
        let input = "@[[Deep/Space]]/(Nested Level)/simple";
        let result = parse_tags(input);
        assert_eq!(
            result[0].tagls,
            vec!["[[Deep/Space]]", "(Nested Level)", "simple"]
        );
    }

    #[test]
    fn test_multiple_tags() {
        let input = "Check @tag1 and @p/[[Project X]]";
        let result = parse_tags(input);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].tagls, vec!["tag1"]);
        assert_eq!(result[1].tagls, vec!["p", "[[Project X]]"]);
    }

    #[test]
    fn test_spaces_without_paranthesis() {
        let input = "@p/Project X";
        let result = parse_tags(input);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].tagls, vec!["p", "Project"]);
    }
}
