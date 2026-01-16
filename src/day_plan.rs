use chrono::NaiveDate;

use crate::block::Block;

pub trait DayPlanTrait {
    fn only_original_blocks(&self) -> Vec<Block>;
    fn with_updated_blocks(self, blocks: &Vec<Block>) -> Self;
    fn day(&self) -> Option<NaiveDate>;
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
    source: Source,
}

impl DayPlan {
    fn only_original_blocks(self) -> Vec<Block> {
        self.blocks
            .iter()
            .cloned()
            .filter(|b| b.origin == self.origin)
            .collect()
    }

    fn with_updated_blocks(self, blocks: &Vec<Block>) -> Self {
        let mut updated_blocks: Vec<Block> = blocks.iter().cloned().collect();
        updated_blocks.sort_by(|a, b| a.period_str.cmp(&b.period_str));
        DayPlan {
            blocks: updated_blocks,
            ..self
        }
    }
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
}
