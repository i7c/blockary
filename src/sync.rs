use crate::day_plan::{self, DayPlan};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;


pub fn find_files(root: &str) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file() &&
                entry.file_name().to_str().unwrap().ends_with(".md"))
        .map(|entry| entry.path().to_path_buf())
        .collect()
}

pub fn day_plans_from_directory(origin: &str, root: &str) -> Vec<DayPlan> {
    let markdown_files = find_files(root);

    let mut dps: Vec<DayPlan> = Vec::new();
    for md_file_path in markdown_files {
        let md_file_path = md_file_path.to_str().unwrap();
        match fs::read_to_string(md_file_path) {
            Ok(c) => {
                println!("Read: {}", md_file_path);
                dps.push(DayPlan::from_markdown(&c, origin, md_file_path, root));
            },
            Err(c) => {
                println!("Could not read file and will ignore: {}", md_file_path);
            }
        }
    }
    dps
}
