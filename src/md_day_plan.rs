use crate::block::Block;
use crate::markdown_access;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct MarkdownDayPlan {
    pub origin: String,
    pub blocks: Vec<Block>,
    pub abs_path: String,
    base_dir: String,
}

impl MarkdownDayPlan {
    pub fn note_id(self: &MarkdownDayPlan) -> String {
        let base_dir = Path::new(&self.base_dir);
        let abs_path = Path::new(&self.abs_path);

        abs_path
            .strip_prefix(base_dir)
            .expect("Base path does not match the absolute path")
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn from_daily_file_md(
        markdown_content: &str,
        origin: &str,
        abs_path: &str,
        base_dir: &str,
    ) -> MarkdownDayPlan {
        let block_strings =
            markdown_access::read_items_under_section(markdown_content, "Time Blocks");
        let blocks = block_strings
            .iter()
            .map(|bs| Block::parse_block_string(origin, bs).expect(""))
            .collect();

        MarkdownDayPlan {
            origin: origin.to_string(),
            blocks: blocks,
            abs_path: abs_path.to_string(),
            base_dir: base_dir.to_string(),
        }
    }

    pub fn write_to_daily_file(&self) {
        let reload_md_content = fs::read_to_string(&self.abs_path);
        match reload_md_content {
            Ok(c) => {
                let section_lines = &self
                    .blocks
                    .iter()
                    .map(|b| b.to_block_string(b.origin != self.origin))
                    .collect();
                let md_with_updated_section =
                    markdown_access::update_section_lines(section_lines, "Time Blocks", &c);

                fs::write(&self.abs_path, md_with_updated_section).expect(
                    "Could not write file. For safety, will cancel all further operations.",
                );
            }
            Err(_) => {
                println!("Skipping: Could not update file {}", self.abs_path);
            }
        }
    }

    pub fn only_original_blocks(self: &MarkdownDayPlan) -> Vec<Block> {
        self.blocks
            .iter()
            .cloned()
            .filter(|b| b.origin == self.origin)
            .collect()
    }

    pub fn with_updated_blocks(self: MarkdownDayPlan, blocks: &Vec<Block>) -> MarkdownDayPlan {
        let mut updated_blocks: Vec<Block> = blocks.iter().cloned().collect();
        updated_blocks.sort_by(|a, b| a.period_str.cmp(&b.period_str));
        MarkdownDayPlan {
            blocks: updated_blocks,
            ..self
        }
    }
}

pub fn original_blocks_from_all(plans: &Vec<MarkdownDayPlan>) -> Vec<Block> {
    let mut result = Vec::new();
    for plan in plans {
        result.extend(plan.only_original_blocks());
    }
    result
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
        let plan =
            MarkdownDayPlan::from_daily_file_md(&markdown, "Personal", "/base/path/a.md", "/base/path");

        assert_eq!(
            plan.blocks,
            vec![
                Block {
                    period_str: "08:00 - 11:00".to_string(),
                    origin: "Personal".to_string(),
                    desc: "This".to_string()
                },
                Block {
                    period_str: "11:00 - 11:30".to_string(),
                    origin: "Personal".to_string(),
                    desc: "should".to_string()
                },
                Block {
                    period_str: "14:00 - 15:00".to_string(),
                    origin: "Personal".to_string(),
                    desc: "appear".to_string()
                },
                Block {
                    period_str: "".to_string(),
                    origin: "Personal".to_string(),
                    desc: "So should this".to_string()
                },
            ],
        );
    }

    #[test]
    fn test_day_plan_file_path() {
        let day_plan = MarkdownDayPlan::from_daily_file_md(
            "",
            "Work",
            "/home/foo/notes/2025/2025-11-12.md",
            "/home/foo/notes",
        );

        assert_eq!(day_plan.note_id(), "2025/2025-11-12.md");
    }

    #[test]
    #[should_panic]
    fn test_day_plan_file_path_with_wrong_base_dir() {
        let day_plan = MarkdownDayPlan::from_daily_file_md(
            "",
            "Work",
            "/home/foo/notes/2025/2025-11-12.md",
            "/home/foo/not-parent",
        );

        day_plan.note_id();
    }

    #[test]
    fn test_get_original_blocks_only() {
        let day_plan = MarkdownDayPlan {
            origin: "Work".to_string(),
            abs_path: "/work/20.md".to_string(),
            base_dir: "/work".to_string(),
            blocks: vec![
                Block {
                    period_str: "08:00 - 10:30".to_string(),
                    origin: "Work".to_string(),
                    desc: "Emails".to_string(),
                },
                Block {
                    period_str: "14:00 - 14:30".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Walk".to_string(),
                },
            ],
        };

        let blocks = day_plan.only_original_blocks();

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks.get(0).unwrap().desc, "Emails");
    }

    #[test]
    fn test_get_original_blocks_from_all() {
        let dp1 = MarkdownDayPlan {
            origin: "Work".to_string(),
            abs_path: "/work/20.md".to_string(),
            base_dir: "/work".to_string(),
            blocks: vec![
                Block {
                    period_str: "08:00 - 10:30".to_string(),
                    origin: "Work".to_string(),
                    desc: "Emails".to_string(),
                },
                Block {
                    period_str: "14:00 - 14:30".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Walk".to_string(),
                },
            ],
        };

        let dp2 = MarkdownDayPlan {
            origin: "Personal".to_string(),
            abs_path: "/work/20.md".to_string(),
            base_dir: "/work".to_string(),
            blocks: vec![
                Block {
                    period_str: "09:00 - 10:00".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Make coffee".to_string(),
                },
                Block {
                    period_str: "14:00 - 14:30".to_string(),
                    origin: "Hobby".to_string(),
                    desc: "Walk".to_string(),
                },
            ],
        };

        let blocks = original_blocks_from_all(&vec![dp1, dp2]);

        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks.get(0).unwrap().origin, "Work");
        assert_eq!(blocks.get(0).unwrap().desc, "Emails");
        assert_eq!(blocks.get(1).unwrap().origin, "Personal");
        assert_eq!(blocks.get(1).unwrap().desc, "Make coffee");
    }

    #[test]
    fn test_update_blocks() {
        let dp1 = MarkdownDayPlan {
            origin: "Work".to_string(),
            abs_path: "/work/20.md".to_string(),
            base_dir: "/work".to_string(),
            blocks: vec![
                Block {
                    period_str: "08:00 - 10:30".to_string(),
                    origin: "Work".to_string(),
                    desc: "Emails".to_string(),
                },
                Block {
                    period_str: "14:00 - 14:30".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Walk".to_string(),
                },
            ],
        };

        let updated = dp1.with_updated_blocks(&vec![Block {
            period_str: "00:00 - 05:30".to_string(),
            origin: "Personal".to_string(),
            desc: "Sleep".to_string(),
        }]);

        assert_eq!(updated.abs_path, "/work/20.md");
        assert_eq!(updated.base_dir, "/work");
        assert_eq!(updated.origin, "Work");
        assert_eq!(updated.blocks.len(), 1);
        assert_eq!(updated.blocks.get(0).unwrap().origin, "Personal");
        assert_eq!(updated.blocks.get(0).unwrap().desc, "Sleep");
    }
}
