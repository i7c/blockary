use crate::blockary_cfg::Config;
use crate::day_plan::DayPlanTrait;
use crate::md_day_plan::MarkdownDayPlan;
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

pub fn day_plans_from_directory(origin: &str, root: &str) -> Vec<MarkdownDayPlan> {
    let markdown_files = find_files(root);

    println!("Loading {} files from {}", markdown_files.len(), origin);

    let mut dps: Vec<MarkdownDayPlan> = Vec::new();
    for md_file_path in markdown_files {
        let md_file_path = md_file_path.to_str().unwrap();
        match fs::read_to_string(md_file_path) {
            Ok(c) => {
                dps.push(MarkdownDayPlan::from_daily_file_md(
                    &c,
                    origin,
                    md_file_path,
                    root,
                ));
            }
            Err(_) => {
                println!("Could not read file and will ignore: {}", md_file_path);
            }
        }
    }
    dps
}

pub fn all_day_plans_from_config(config: Config) -> Vec<MarkdownDayPlan> {
    config
        .origins
        .iter()
        .map(|(_, origin)| day_plans_from_directory(&origin.name, &origin.path))
        .flatten()
        .collect()
}

pub fn day_plans_by_day(
    all_day_plans: Vec<MarkdownDayPlan>,
) -> HashMap<NaiveDate, Vec<MarkdownDayPlan>> {
    let mut day_plans_by_note_id: HashMap<NaiveDate, Vec<MarkdownDayPlan>> = HashMap::new();
    for dp in all_day_plans {
        let Some(key) = dp.day() else { continue };
        day_plans_by_note_id
            .entry(key)
            .or_insert(Vec::new())
            .push(dp);
    }

    day_plans_by_note_id
}
