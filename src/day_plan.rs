use crate::block::Block;
use crate::markdown_access;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DayPlan {
    pub origin: String,
    pub blocks: Vec<Block>,
    pub abs_path: String,
    base_dir: String,
}

impl DayPlan {
    pub fn note_id(self: &DayPlan) -> String {
        let base_dir = Path::new(&self.base_dir);
        let abs_path = Path::new(&self.abs_path);

        abs_path
            .strip_prefix(base_dir)
            .expect("Base path does not match the absolute path")
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn from_markdown(
        markdown_content: &str,
        origin: &str,
        abs_path: &str,
        base_dir: &str,
    ) -> DayPlan {
        let block_strings =
            markdown_access::read_items_under_section(markdown_content, "Time Blocks");
        let blocks = block_strings
            .iter()
            .map(|bs| Block::parse(origin, bs).expect(""))
            .collect();

        DayPlan {
            origin: origin.to_string(),
            blocks: blocks,
            abs_path: abs_path.to_string(),
            base_dir: base_dir.to_string(),
        }
    }

    pub fn update_markdown(self: &DayPlan, markdown_content: &str) -> String {
        let block_strings: Vec<String> = self
            .blocks
            .iter()
            .map(|b| b.to_block_string(b.origin != self.origin))
            .collect();

        markdown_access::update_section_lines(&block_strings, "Time Blocks", markdown_content)
    }

    pub fn only_original_blocks(self: &DayPlan) -> Vec<Block> {
        self.blocks
            .iter()
            .cloned()
            .filter(|b| b.origin == self.origin)
            .collect()
    }

    pub fn sort_blocks(self: &mut DayPlan) {
        self.blocks.sort_by(|a, b| a.period.cmp(&b.period));
    }

    pub fn set_origin(self: &mut DayPlan, origin: &str) {
        self.origin = origin.to_string();
    }

    pub fn with_updated_blocks(self: DayPlan, blocks: &Vec<Block>) -> DayPlan {
        DayPlan {
            origin: self.origin,
            blocks: blocks.iter().cloned().collect(),
            abs_path: self.abs_path,
            base_dir: self.base_dir,
        }
    }

    pub fn merge(self: DayPlan, other: DayPlan) -> DayPlan {
        let mut blocks = self.blocks;
        blocks.extend(other.blocks);
        blocks.sort_by(|a, b| a.period.cmp(&b.period));
        DayPlan {
            origin: self.origin,
            blocks: blocks,
            abs_path: self.abs_path,
            base_dir: self.base_dir,
        }
    }
}

pub fn original_blocks_from_all(plans: &Vec<DayPlan>) -> Vec<Block> {
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
        let plan = DayPlan::from_markdown(&markdown, "Personal", "/base/path/a.md", "/base/path");

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
            abs_path: "/a/b".to_string(),
            base_dir: "/a".to_string(),
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
            abs_path: "/b/b".to_string(),
            base_dir: "/b".to_string(),
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

    #[test]
    fn test_update_block_strings_in_markdown() {
        let markdown = "
# Some Title
## Some other section
bla foo
## Time Blocks
- 08:00 - 11:00 This
- 11:00 - 11:30 should
- 14:00 - 15:00 appear
# Notes
- 10:00 - 11:00 This should not appear in the result
";
        let mut plan = DayPlan::from_markdown(&markdown, "Personal", "/a/b", "/a");
        plan.blocks.push(Block {
            period: "12:00".to_string(),
            origin: "Work".to_string(),
            desc: "Lunch".to_string(),
        });
        plan.sort_blocks();

        assert_eq!(
            plan.update_markdown(markdown),
            "
# Some Title
## Some other section
bla foo
## Time Blocks
- 08:00 - 11:00 This
- 11:00 - 11:30 should
- 12:00 (Work) Lunch
- 14:00 - 15:00 appear
# Notes
- 10:00 - 11:00 This should not appear in the result
"
        );
    }

    #[test]
    fn test_day_plan_file_path() {
        let day_plan = DayPlan::from_markdown(
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
        let day_plan = DayPlan::from_markdown(
            "",
            "Work",
            "/home/foo/notes/2025/2025-11-12.md",
            "/home/foo/not-parent",
        );

        day_plan.note_id();
    }

    #[test]
    fn test_get_original_blocks_only() {
        let day_plan = DayPlan {
            origin: "Work".to_string(),
            abs_path: "/work/20.md".to_string(),
            base_dir: "/work".to_string(),
            blocks: vec![
                Block {
                    period: "08:00 - 10:30".to_string(),
                    origin: "Work".to_string(),
                    desc: "Emails".to_string(),
                },
                Block {
                    period: "14:00 - 14:30".to_string(),
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
        let dp1 = DayPlan {
            origin: "Work".to_string(),
            abs_path: "/work/20.md".to_string(),
            base_dir: "/work".to_string(),
            blocks: vec![
                Block {
                    period: "08:00 - 10:30".to_string(),
                    origin: "Work".to_string(),
                    desc: "Emails".to_string(),
                },
                Block {
                    period: "14:00 - 14:30".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Walk".to_string(),
                },
            ],
        };

        let dp2 = DayPlan {
            origin: "Personal".to_string(),
            abs_path: "/work/20.md".to_string(),
            base_dir: "/work".to_string(),
            blocks: vec![
                Block {
                    period: "09:00 - 10:00".to_string(),
                    origin: "Personal".to_string(),
                    desc: "Make coffee".to_string(),
                },
                Block {
                    period: "14:00 - 14:30".to_string(),
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
}
