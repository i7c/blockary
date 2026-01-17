use crate::blockary_cfg;
use crate::cal_day_plan;
use crate::day_plan;
use crate::day_plan::DayPlanTrait;
use crate::sync;
use chrono::Local;
use clap::{Parser, Subcommand};
use std::env;
use std::fs;

#[derive(Parser)]
#[command(name = "blockary")]
#[command(about = "Synchronize time blocks across dayplans and calendars", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync between local markdown dayplan files
    Sync {
        #[arg(short, long)]
        ics_file: Option<String>,
    },
}

pub fn run() {
    let args = Cli::parse();

    match args.command {
        Commands::Sync { ics_file } => {
            let mut config_path = env::home_dir()
                .expect("$HOME is not set")
                .into_os_string()
                .into_string()
                .unwrap();
            config_path.push_str("/.config/blockary.toml");
            let config = fs::read_to_string(config_path).expect("Could not read config file");
            let config = blockary_cfg::load(&config);

            let today = Local::now().date_naive();

            let all_day_plans = sync::all_day_plans_from_config(config);
            let day_plans_by_note_id = sync::day_plans_by_day(all_day_plans);

            let sync_count = day_plans_by_note_id
                .iter()
                .filter(|(_id, day_plans)| day_plans.len() > 1)
                .count();
            println!(
                "{sync_count} of {} days will be synced",
                day_plans_by_note_id.len()
            );

            for (_id, plans) in day_plans_by_note_id {
                let mut synced_blocks = day_plan::original_blocks_from_all(&plans);

                if let Some(ref ics_file) = ics_file {
                    let dp = plans.get(0).unwrap();
                    if let Some(date) = dp.day() {
                        if today == date {
                            if let Ok(ical_content) = fs::read_to_string(&ics_file) {
                                if let Ok(cal_plan) =
                                    cal_day_plan::CalDayPlan::from_icalendar(&ical_content, today)
                                {
                                    synced_blocks.extend(cal_plan.blocks);
                                }
                            } else {
                                println!("Could not open ICS file, continue without ...");
                            }
                        }
                    }
                }

                for plan in plans {
                    plan.with_updated_blocks(&synced_blocks)
                        .write_to_daily_file();
                }
            }
        }
    }
}
