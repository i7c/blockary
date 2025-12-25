use day_plan::DayPlan;
use std::env;
use std::fs;

mod block;
mod blockary_cfg;
mod day_plan;
mod markdown_access;
mod sync;

fn main() {
    let mut config_path = env::home_dir()
        .expect("$HOME is not set")
        .into_os_string()
        .into_string()
        .unwrap();
    config_path.push_str("/.config/blockary.toml");
    let config = fs::read_to_string(config_path).expect("Could not read config file");
    let config = blockary_cfg::load(&config);

    let all_day_plans = sync::all_day_plans_from_config(config);
    let day_plans_by_note_id = sync::day_plans_by_note_id(all_day_plans);

    for (_id, plans) in day_plans_by_note_id {
        let synced_blocks = day_plan::original_blocks_from_all(&plans);
        for plan in plans {
            let markdown_content = fs::read_to_string(&plan.abs_path);
            match markdown_content {
                Ok(c) => {
                    let updated_markdown_content = plan.with_updated_blocks(&synced_blocks);
                    fs::write(
                        &updated_markdown_content.abs_path,
                        updated_markdown_content.update_markdown(&c),
                    )
                    .expect(
                        "Could not write file. For safety, will cancel all further operations.",
                    );
                }
                Err(_) => {
                    println!("Skipping: Could not update file {}", plan.abs_path);
                }
            }
        }
    }
}
