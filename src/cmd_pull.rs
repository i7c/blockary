use chrono::NaiveDate;

use crate::{
    blockary_cfg::{Config, Dir},
    cal_day_plan::day_plan_from_ical,
    day_plan::{DayPlanRepo, DayPlanRepoType},
};

pub fn command(config: Config, for_day: &NaiveDate, target: Option<String>) {
    let cals = match &config.cals {
        Some(cals) if !cals.is_empty() => cals,
        _ => {
            eprintln!("Error: No calendars configured. Add [cals] entries to your blockary.toml.");
            return;
        }
    };

    let target_dir = match resolve_target_dir(&config, target.as_deref()) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let date_str = for_day.format("%Y-%m-%d").to_string();

    let repo = DayPlanRepo {
        name: target_dir.name.clone(),
        repo_type: DayPlanRepoType::MarkdownDirectory {
            dir: target_dir.path.clone(),
        },
    };

    let day_plans = repo.all_of_day(for_day);
    if day_plans.is_empty() {
        println!(
            "Warning: No file found for {} in '{}' ({}). Skipping.",
            date_str, target_dir.name, target_dir.path
        );
        return;
    }
    let mut existing_plan = day_plans.into_iter().next().unwrap();

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

        let mut new_blocks_added = 0;
        for cal_block in &cal_day_plan.blocks {
            let conflict = existing_plan
                .blocks
                .iter()
                .any(|b| b.period_str == cal_block.period_str);

            if conflict {
                println!(
                    "  Warning: Skipping '{}' ({}) — a block at {} already exists.",
                    cal_block.desc, cal_name, cal_block.period_str
                );
            } else {
                existing_plan.blocks.push(cal_block.clone());
                new_blocks_added += 1;
            }
        }

        if new_blocks_added > 0 {
            println!("  Added {} block(s) from '{}'.", new_blocks_added, cal_name);
        }
    }

    existing_plan
        .blocks
        .sort_by(|a, b| a.period_str.cmp(&b.period_str));
    existing_plan.write_to_daily_file();
    println!("Written to '{}'.", target_dir.name);
}

fn resolve_target_dir<'a>(config: &'a Config, target: Option<&str>) -> Result<&'a Dir, String> {
    match target {
        Some(key) => config.dirs.get(key).ok_or_else(|| {
            let available: Vec<&str> = config.dirs.keys().map(|k| k.as_str()).collect();
            format!(
                "Unknown target '{}'. Available: {}",
                key,
                available.join(", ")
            )
        }),
        None => {
            if config.dirs.len() == 1 {
                Ok(config.dirs.values().next().unwrap())
            } else {
                let available: Vec<&str> = config.dirs.keys().map(|k| k.as_str()).collect();
                Err(format!(
                    "Multiple directories configured, specify --target. Available: {}",
                    available.join(", ")
                ))
            }
        }
    }
}
