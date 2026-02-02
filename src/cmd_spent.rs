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

pub fn time_spent_in_origin(today: chrono::NaiveDate, origin: &blockary_cfg::Dir) {
    let repo = DayPlanRepo {
        name: origin.name.clone(),
        repo_type: day_plan::DayPlanRepoType::MarkdownDirectory {
            dir: origin.path.clone(),
        },
    };

    let all_of_day = repo.all_of_day(today);

    let (hours, minutes) =
        time_summary::minutes_to_hours_minutes(time_summary::total_time_spent(all_of_day));
    println!("--:--");
    println!("{:02}:{:02} on {} today", hours, minutes, origin.name);
}
