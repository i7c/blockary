use crate::blockary_cfg::Config;
use crate::day_plan::DayPlan;
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

pub fn day_plans_from_directory(origin: &str, root: &str) -> Vec<DayPlan> {
    let markdown_files = find_files(root);

    println!("Loading {} files from {}", markdown_files.len(), origin);

    let mut dps: Vec<DayPlan> = Vec::new();
    for md_file_path in markdown_files {
        let md_file_path = md_file_path.to_str().unwrap();
        match fs::read_to_string(md_file_path) {
            Ok(c) => {
                dps.push(DayPlan::from_daily_file_md(&c, origin, md_file_path, root));
            }
            Err(_) => {
                println!("Could not read file and will ignore: {}", md_file_path);
            }
        }
    }
    dps
}

pub fn all_day_plans_from_config(config: Config) -> Vec<DayPlan> {
    config
        .origins
        .iter()
        .map(|(_, origin)| day_plans_from_directory(&origin.name, &origin.path))
        .flatten()
        .collect()
}

pub fn day_plans_by_note_id(all_day_plans: Vec<DayPlan>) -> HashMap<String, Vec<DayPlan>> {
    let mut day_plans_by_note_id: HashMap<String, Vec<DayPlan>> = HashMap::new();
    for dp in all_day_plans {
        day_plans_by_note_id
            .entry(dp.note_id())
            .or_insert(Vec::new())
            .push(dp);
    }
    let mut sync_groups_with_multiple = 0;
    for (_, plans) in &day_plans_by_note_id {
        if plans.len() > 1 {
            sync_groups_with_multiple += 1;
        }
    }

    println!("-> {} synchronization groups", day_plans_by_note_id.len());
    println!("-> {} matches", sync_groups_with_multiple);
    day_plans_by_note_id
}
