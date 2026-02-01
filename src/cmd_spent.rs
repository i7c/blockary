use crate::blockary_cfg;
use crate::day_plan;
use crate::day_plan::DayPlanRepo;

fn minutes_to_hours_minutes(total_duration_today: u16) -> (u16, u16) {
    let hours = total_duration_today / 60;
    let minutes = total_duration_today % 60;
    (hours, minutes)
}

pub fn time_spent_in_origin(today: chrono::NaiveDate, origin: &blockary_cfg::Dir) {
    let repo = DayPlanRepo {
        name: origin.name.clone(),
        repo_type: day_plan::DayPlanRepoType::MarkdownDirectory {
            dir: origin.path.clone(),
        },
    };

    let total_duration_today = repo.all_of_day(today).iter().fold(0, |total_duration, dp| {
        total_duration
            + dp.only_original_blocks().iter().fold(0, |acc, b| {
                let (h, m) = minutes_to_hours_minutes(b.duration);

                if b.tags
                    .iter()
                    .any(|tag| match tag.tagls.get(0).map(|s| s.as_ref()) {
                        Some("break") => true,
                        _ => false,
                    })
                {
                    println!("{:02}:{:02} - {} (IGNORED)", h, m, b.desc);
                    acc
                } else {
                    println!("{:02}:{:02} - {}", h, m, b.desc);
                    acc + b.duration
                }
            })
    });

    let (hours, minutes) = minutes_to_hours_minutes(total_duration_today);
    println!("--:--");
    println!("{:02}:{:02} on {} today", hours, minutes, origin.name);
}
