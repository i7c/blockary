use chrono::NaiveDate;
use std::fs;
use walkdir::WalkDir;

use crate::{
    blockary_cfg::Config,
    cal_day_plan::day_plan_from_ical,
    day_plan::day_plan_from_daily_file_md,
};

pub fn command(config: Config, for_day: &NaiveDate) {
    let cals = match &config.cals {
        Some(cals) if !cals.is_empty() => cals,
        _ => {
            eprintln!("Error: No calendars configured. Add [cals] entries to your blockary.toml.");
            return;
        }
    };

    let date_str = for_day.format("%Y-%m-%d").to_string();

    for (cal_name, cal) in cals {
        println!("Pulling from calendar '{}' ({})...", cal_name, cal.uri);

        let ical_content = match reqwest::blocking::get(&cal.uri) {
            Ok(resp) => match resp.text() {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("Error: Could not read response from '{}': {}", cal.uri, e);
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Error: Could not fetch calendar '{}': {}", cal.uri, e);
                continue;
            }
        };

        let cal_day_plan = day_plan_from_ical(&ical_content, *for_day, cal_name);

        if cal_day_plan.blocks.is_empty() {
            println!("  No events found for {} in calendar '{}'.", date_str, cal_name);
            continue;
        }

        for (_, dir) in &config.dirs {
            match find_md_file_for_day(&dir.path, &date_str) {
                None => {
                    println!(
                        "  Warning: No file found for {} in '{}' ({}). Skipping.",
                        date_str, dir.name, dir.path
                    );
                }
                Some(abs_path) => {
                    let md_content = match fs::read_to_string(&abs_path) {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!(
                                "  Warning: Could not read {}: {}. Skipping.",
                                abs_path, e
                            );
                            continue;
                        }
                    };

                    let mut existing_plan =
                        day_plan_from_daily_file_md(&md_content, &dir.name, &abs_path, &dir.path);

                    let mut new_blocks_added = 0;
                    for cal_block in &cal_day_plan.blocks {
                        let conflict = existing_plan
                            .blocks
                            .iter()
                            .any(|b| b.period_str == cal_block.period_str);

                        if conflict {
                            println!(
                                "  Warning: Skipping '{}' ({}) in '{}' — a block at {} already exists.",
                                cal_block.desc, cal_name, dir.name, cal_block.period_str
                            );
                        } else {
                            existing_plan.blocks.push(cal_block.clone());
                            new_blocks_added += 1;
                        }
                    }

                    if new_blocks_added > 0 {
                        existing_plan
                            .blocks
                            .sort_by(|a, b| a.period_str.cmp(&b.period_str));
                        existing_plan.write_to_daily_file();
                        println!(
                            "  Added {} block(s) to '{}' ({}).",
                            new_blocks_added, dir.name, abs_path
                        );
                    }
                }
            }
        }
    }
}

fn find_md_file_for_day(root: &str, date_str: &str) -> Option<String> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_file()
                && entry
                    .file_name()
                    .to_str()
                    .map(|n| n.ends_with(".md"))
                    .unwrap_or(false)
                && entry
                    .path()
                    .to_str()
                    .map(|p| p.contains(date_str))
                    .unwrap_or(false)
        })
        .map(|entry| entry.path().to_str().unwrap().to_string())
        .next()
}
