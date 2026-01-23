use crate::blockary_cfg;
use crate::day_plan;
use crate::sync::Sync;
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
        Commands::Sync { .. } => {
            let config = load_configuration();
            let sync = Sync::from_config(&config);

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
    }
}

fn load_configuration() -> blockary_cfg::Config {
    let mut config_path = env::home_dir()
        .expect("$HOME is not set")
        .into_os_string()
        .into_string()
        .unwrap();
    config_path.push_str("/.config/blockary.toml");
    let config = fs::read_to_string(config_path).expect("Could not read config file");
    let config = blockary_cfg::load(&config);
    config
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
