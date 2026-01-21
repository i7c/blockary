use crate::blockary_cfg::Config;
use crate::day_plan::{self, DayPlan};
use chrono::NaiveDate;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

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

pub fn all_day_plans_from_config(config: Config) -> Vec<DayPlan> {
    config
        .origins
        .iter()
        .map(|(_, origin)| {
            let repo = day_plan::DayPlanRepo {
                name: origin.name.clone(),
                repo_type: day_plan::DayPlanRepoType::MarkdownDirectory {
                    dir: origin.path.clone(),
                },
            };
            repo.all()
        })
        .flatten()
        .collect()
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
