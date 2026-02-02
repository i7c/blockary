use crate::day_plan;

pub fn minutes_to_hours_minutes(total_duration_today: u16) -> (u16, u16) {
    let hours = total_duration_today / 60;
    let minutes = total_duration_today % 60;
    (hours, minutes)
}

pub fn total_time_spent(all_of_day: Vec<day_plan::DayPlan>) -> u16 {
    let total_duration_today = all_of_day.iter().fold(0, |total_duration, dp| {
        total_duration
            + dp.only_original_blocks().iter().fold(0, |acc, b| {
                if b.tags
                    .iter()
                    .any(|tag| match tag.tagls.get(0).map(|s| s.as_ref()) {
                        Some("break") => true,
                        _ => false,
                    })
                {
                    acc
                } else {
                    acc + b.duration
                }
            })
    });
    total_duration_today
}
