use crate::blockary_cfg;
use crate::day_plan;
use crate::sync::Sync;

pub fn command(config: &blockary_cfg::Config) {
    let sync = Sync::from_config(config);
    let day_plans_by_note_id = sync.all_day_plans_by_day();

    print_sync_stats(&day_plans_by_note_id);

    for (_id, plans) in day_plans_by_note_id {
        let synced_blocks = day_plan::original_blocks_from_all(&plans);
        for plan in plans {
            plan.with_updated_blocks(&synced_blocks)
                .write_to_daily_file();
        }
    }
}

fn print_sync_stats(
    day_plans_by_note_id: &std::collections::HashMap<chrono::NaiveDate, Vec<day_plan::DayPlan>>,
) {
    let sync_count = day_plans_by_note_id
        .iter()
        .filter(|(_id, day_plans)| day_plans.len() > 1)
        .count();
    println!(
        "{sync_count} of {} days will be synced",
        day_plans_by_note_id.len()
    );
}
