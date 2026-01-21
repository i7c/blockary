use crate::blockary_cfg::Config;
use crate::day_plan::{self, DayPlan};
use chrono::NaiveDate;
use std::collections::HashMap;

pub fn all_day_plans_from_config(config: Config) -> Vec<DayPlan> {
    let mut day_plans = Vec::new();


    for (_, origin) in &config.dirs {
        let repo = day_plan::DayPlanRepo {
            name: origin.name.clone(),
            repo_type: day_plan::DayPlanRepoType::MarkdownDirectory {
                dir: origin.path.clone(),
            },
        };
        day_plans.extend(repo.all());
    }
    day_plans
}

pub fn day_plans_by_day(all_day_plans: Vec<DayPlan>) -> HashMap<NaiveDate, Vec<DayPlan>> {
    let mut day_plans_by_note_id: HashMap<NaiveDate, Vec<DayPlan>> = HashMap::new();
    for dp in all_day_plans {
        let Some(key) = dp.day() else { continue };
        day_plans_by_note_id
            .entry(key)
            .or_insert(Vec::new())
            .push(dp);
    }

    day_plans_by_note_id
}
