use std::{fs, path::PathBuf, str::FromStr};

use chrono::NaiveDate;
use regex::Regex;
use walkdir::WalkDir;

use crate::{block::Block, markdown_access};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DayPlanRepoType {
    MarkdownDirectory { dir: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayPlanRepo {
    pub name: String,
    pub repo_type: DayPlanRepoType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    ObsMarkDown { abs_path: String, base_dir: String },
    ICalendar,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayPlan {
    pub origin: String,
    pub blocks: Vec<Block>,
    pub day: Option<NaiveDate>,
    pub source: Source,
}

impl DayPlanRepo {
    pub fn all(&self) -> Vec<DayPlan> {
        match &self.repo_type {
            DayPlanRepoType::MarkdownDirectory { dir } => {
                day_plans_from_directory(&self.name, &dir)
            }
        }
    }
}

impl DayPlan {
    pub fn only_original_blocks(&self) -> Vec<Block> {
        self.blocks
            .iter()
            .cloned()
            .filter(|b| b.origin == self.origin)
            .collect()
    }

    pub fn with_updated_blocks(self, blocks: &Vec<Block>) -> Self {
        let mut updated_blocks: Vec<Block> = blocks.iter().cloned().collect();
        updated_blocks.sort_by(|a, b| a.period_str.cmp(&b.period_str));
        DayPlan {
            blocks: updated_blocks,
            ..self
        }
    }

    pub fn day(&self) -> Option<NaiveDate> {
        match self.day {
            Some(_) => return self.day.clone(),
            _ => {
                if let Source::ObsMarkDown { abs_path, base_dir } = &self.source {
                    maybe_extract_day_from_path(abs_path, base_dir)
                } else {
                    None
                }
            }
        }
    }

    pub fn write_to_daily_file(&self) {
        let Source::ObsMarkDown {
            abs_path,
            base_dir: _,
        } = &self.source
        else {
            return;
        };

        let reload_md_content = fs::read_to_string(&abs_path);
        match reload_md_content {
            Ok(c) => {
                let section_lines = &self
                    .blocks
                    .iter()
                    .map(|b| b.to_block_string(b.origin != self.origin))
                    .map(|bs| format!("- {}", bs))
                    .collect();
                let md_with_updated_section =
                    markdown_access::update_section_lines(section_lines, "Time Blocks", &c);

                fs::write(&abs_path, md_with_updated_section).expect(
                    "Could not write file. For safety, will cancel all further operations.",
                );
            }
            Err(_) => {
                println!("Skipping: Could not update file {}", &abs_path);
            }
        }
    }
}

fn find_files(root: &str) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_file() && entry.file_name().to_str().unwrap().ends_with(".md")
        })
        .map(|entry| entry.path().to_path_buf())
        .collect()
}

pub fn day_plan_from_daily_file_md(
    markdown_content: &str,
    origin: &str,
    abs_path: &str,
    base_dir: &str,
) -> DayPlan {
    let block_strings = markdown_access::read_items_under_section(markdown_content, "Time Blocks");
    let blocks = block_strings
        .iter()
        .map(|bs| Block::parse_block_string(origin, bs).expect(""))
        .collect();

    DayPlan {
        origin: origin.to_string(),
        blocks: blocks,
        source: Source::ObsMarkDown {
            abs_path: abs_path.to_string(),
            base_dir: base_dir.to_string(),
        },
        day: None,
    }
}

pub fn day_plans_from_directory(origin: &str, root: &str) -> Vec<DayPlan> {
    let markdown_files = find_files(root);

    println!("Loading {} files from {}", markdown_files.len(), origin);

    let mut dps: Vec<DayPlan> = Vec::new();
    for md_file_path in markdown_files {
        let md_file_path = md_file_path.to_str().unwrap();
        match fs::read_to_string(md_file_path) {
            Ok(c) => {
                dps.push(day_plan_from_daily_file_md(&c, origin, md_file_path, root));
            }
            Err(_) => {
                println!("Could not read file and will ignore: {}", md_file_path);
            }
        }
    }
    dps
}

fn maybe_extract_day_from_path(abs_path: &String, base_dir: &String) -> Option<NaiveDate> {
    let relative_path = abs_path
        .strip_prefix(base_dir)
        .expect("Base path does not match the absolute path")
        .to_string();

    match Regex::new(r"\d\d\d\d-\d\d-\d\d")
        .unwrap()
        .captures(&relative_path)
    {
        Some(matches) => {
            let date_str: String = matches.get(0).map(|m| m.as_str().to_string())?;
            match NaiveDate::from_str(&date_str) {
                Ok(d) => Some(d),
                Err(_) => None,
            }
        }
        None => None,
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
    fn test_update_blocks() {
        let dp1 = DayPlan {
            origin: "Work".to_string(),
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
            day: None,
            source: Source::ObsMarkDown {
                abs_path: "/work/20.md".to_string(),
                base_dir: "/work".to_string(),
            },
        };

        let updated = dp1.with_updated_blocks(&vec![Block {
            period_str: "00:00 - 05:30".to_string(),
            origin: "Personal".to_string(),
            desc: "Sleep".to_string(),
        }]);

        assert_eq!(
            updated.source,
            Source::ObsMarkDown {
                abs_path: "/work/20.md".to_string(),
                base_dir: "/work".to_string()
            }
        );
        assert_eq!(updated.origin, "Work");
        assert_eq!(updated.blocks.len(), 1);
        assert_eq!(updated.blocks.get(0).unwrap().origin, "Personal");
        assert_eq!(updated.blocks.get(0).unwrap().desc, "Sleep");
    }

    #[test]
    fn test_get_original_blocks_only() {
        let day_plan = DayPlan {
            origin: "Work".to_string(),
            day: None,
            source: Source::ObsMarkDown {
                abs_path: "/work/20.md".to_string(),
                base_dir: "/work".to_string(),
            },
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
    fn test_get_day_from_path() {
        let day_plan = DayPlan {
            origin: "Work".to_string(),
            day: None,
            source: Source::ObsMarkDown {
                abs_path: "/work/2015-11-03.md".to_string(),
                base_dir: "/work".to_string(),
            },
            blocks: vec![],
        };

        assert_eq!(
            day_plan.day(),
            Some(NaiveDate::from_ymd_opt(2015, 11, 03).unwrap())
        );
    }

    #[test]
    fn test_get_day_takes_precedence_from_day() {
        let day_plan = DayPlan {
            origin: "Work".to_string(),
            day: Some(NaiveDate::from_ymd_opt(2020, 10, 20).unwrap()),
            source: Source::ObsMarkDown {
                abs_path: "/work/2015-11-03.md".to_string(),
                base_dir: "/work".to_string(),
            },
            blocks: vec![],
        };

        assert_eq!(
            day_plan.day(),
            Some(NaiveDate::from_ymd_opt(2020, 10, 20).unwrap())
        );
    }

    #[test]
    fn test_get_no_day_can_be_calculated() {
        let day_plan = DayPlan {
            origin: "Work".to_string(),
            day: None,
            source: Source::ObsMarkDown {
                abs_path: "/work/20250103.md".to_string(),
                base_dir: "/work".to_string(),
            },
            blocks: vec![],
        };

        assert_eq!(day_plan.day(), None);
    }

    #[test]
    fn test_get_original_blocks_from_all() {
        let dp1 = DayPlan {
            origin: "Work".to_string(),
            day: None,
            source: Source::ObsMarkDown {
                abs_path: "/work/2015-11-03.md".to_string(),
                base_dir: "/work".to_string(),
            },
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

        let dp2 = DayPlan {
            origin: "Personal".to_string(),
            day: None,
            source: Source::ObsMarkDown {
                abs_path: "/work/2015-11-03.md".to_string(),
                base_dir: "/work".to_string(),
            },
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
}
