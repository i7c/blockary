use crate::blockary_cfg::Config;
use crate::day_plan::{self, DayPlan, DayPlanRepo};
use chrono::NaiveDate;
use std::collections::HashMap;

pub struct Sync {
    pub repos: Vec<DayPlanRepo>,
}

impl Sync {
    pub fn from_config(config: &Config) -> Self {
        let mut repos = Vec::new();

        for (_, origin) in &config.dirs {
            println!("Load {} ({})", origin.name, origin.path);
            let repo = day_plan::DayPlanRepo {
                name: origin.name.clone(),
                repo_type: day_plan::DayPlanRepoType::MarkdownDirectory {
                    dir: origin.path.clone(),
                },
            };
            repos.push(repo);
        }
        Sync { repos }
    }

    pub fn all_day_plans(&self) -> Vec<DayPlan> {
        let mut day_plans = Vec::new();

        for repo in &self.repos {
            day_plans.extend(repo.all());
        }
        day_plans
    }

    pub fn all_day_plans_by_day(&self) -> HashMap<NaiveDate, Vec<DayPlan>> {
        let mut day_plans_by_note_id: HashMap<NaiveDate, Vec<DayPlan>> = HashMap::new();
        for dp in self.all_day_plans() {
            let Some(key) = dp.day() else { continue };
            day_plans_by_note_id
                .entry(key)
                .or_insert(Vec::new())
                .push(dp);
        }
        day_plans_by_note_id
    }
}
