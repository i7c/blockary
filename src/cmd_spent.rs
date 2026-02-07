use crate::block::Block;
use crate::blockary_cfg;
use crate::day_plan;
use crate::day_plan::DayPlanRepo;
use crate::time_summary;

pub fn command(config: blockary_cfg::Config, today: chrono::NaiveDate) {
    for (_, dir) in &config.dirs {
        println!("\n> {}", dir.name);
        time_spent_in_origin(today, dir);
    }
}


pub fn time_spent_in_origin(from_inclusive: chrono::NaiveDate, origin: &blockary_cfg::Dir) {
    let repo = DayPlanRepo {
        name: origin.name.clone(),
        repo_type: day_plan::DayPlanRepoType::MarkdownDirectory {
            dir: origin.path.clone(),
        },
    };
    let all_in_range = repo.all_of_day(from_inclusive);

    let mut all_blocks: Vec<&Block> = Vec::new();
    for dp in &all_in_range {
        for block in &dp.blocks {
            all_blocks.push(&block);
        }
    }
    time_summary::time_per_tag(&all_blocks, 0);

    
    
    let (hours, minutes) =
        time_summary::minutes_to_hours_minutes(time_summary::total_time_spent(&all_in_range));
    println!("--:--");
    println!("{:02}:{:02} on {} today", hours, minutes, origin.name);
}
