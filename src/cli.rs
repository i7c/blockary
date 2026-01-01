use crate::blockary_cfg;
use crate::day_plan::DayPlan;
use crate::md_day_plan;
use crate::sync;
use std::env;
use std::fs;

pub fn run() {
    if let Some(command) = env::args().nth(1) {
        let command: &str = &command;
        match command {
            "sync" => {
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
                    let synced_blocks = md_day_plan::original_blocks_from_all(&plans);
                    for plan in plans {
                        plan.with_updated_blocks(&synced_blocks)
                            .write_to_daily_file();
                    }
                }
            }
            _ => {
                println!("Unknown command {}", command);
                return;
            }
        }
    } else {
        println!("Specify a command");
    }
}
