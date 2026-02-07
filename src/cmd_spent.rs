use chrono::NaiveDate;
use comfy_table::Table;
use comfy_table::presets;

use crate::block::Block;
use crate::blockary_cfg;
use crate::day_plan;
use crate::day_plan::DayPlanRepo;
use crate::time_summary;
use crate::time_summary::minutes_to_hours_minutes;

pub fn command(config: blockary_cfg::Config, today: chrono::NaiveDate) {
    for (_, dir) in &config.dirs {
        println!("\n> {}", dir.name);
        time_spent_per_origin(today, dir);
    }
}

pub fn time_spent_per_origin(from_inclusive: chrono::NaiveDate, origin: &blockary_cfg::Dir) {
    let repo = DayPlanRepo {
        name: origin.name.clone(),
        repo_type: day_plan::DayPlanRepoType::MarkdownDirectory {
            dir: origin.path.clone(),
        },
    };
    let all_in_range = repo.all_of_day(from_inclusive);

    let mut all_blocks: Vec<&Block> = Vec::new();
    for dp in &all_in_range {
        for block in &dp.only_original_blocks_slice() {
            all_blocks.push(&block);
        }
    }
    let tag_timings = time_summary::time_per_tag(&all_blocks, 0);
    let mut table = Table::new();

    table.set_header(vec!["Tagl", "..", "..", "Time", "%"]);
    table.load_preset(presets::UTF8_FULL_CONDENSED);
    add_row_for_tagl(&tag_timings, &mut table, 0);

    println!("{table}");

    let (hours, minutes) =
        time_summary::minutes_to_hours_minutes(time_summary::total_time_spent(&all_in_range));
    println!("--:--");
    println!("{:02}:{:02} on {} today", hours, minutes, origin.name);
}

fn add_row_for_tagl(tag_timings: &Vec<time_summary::TagTime>, table: &mut Table, level: u8) {
    let total = tag_timings.iter().fold(0, |acc, tt| acc + tt.minutes);

    for tt in tag_timings {
        let mut rowc = Vec::new();
        for _ in 0..level {
            rowc.push("".to_string());
        }
        rowc.push(tt.tag.clone());
        for _ in level..2 {
            rowc.push("".to_string());
        }
        // time
        let (hours, minutes) = minutes_to_hours_minutes(tt.minutes);
        rowc.push(format!("{:02}:{:02}", hours, minutes));

        // %
        rowc.push(format!("{:3}%", (tt.minutes as u32 * 100) / total as u32));

        table.add_row(rowc);
        add_row_for_tagl(&tt.sub_tags, table, level + 1);

        if level == 0 {
            table.add_row(vec![""]);
        }
    }
}
