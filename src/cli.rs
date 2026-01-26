use crate::blockary_cfg;
use crate::day_plan;
use crate::day_plan::DayPlanRepo;
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
    Spent {
        #[arg(short, long)]
        origin: String,
    },
}

pub fn run() {
    let args = Cli::parse();
    let config = load_configuration();
    let today = chrono::Local::now().date_naive();

    match args.command {
        Commands::Sync { .. } => {
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
        Commands::Spent { origin } => {
            let Some(origin) = config.dirs.get(&origin) else {
                println!("Fatal: No such origin {origin}.");
                return;
            };

            let repo = DayPlanRepo {
                name: origin.name.clone(),
                repo_type: day_plan::DayPlanRepoType::MarkdownDirectory {
                    dir: origin.path.clone(),
                },
            };

            let total_duration_today =
                repo.all_of_day(today).iter().fold(0, |total_duration, dp| {
                    total_duration
                        + dp.only_original_blocks().iter().fold(0, |acc, b| {
                            let (h, m) = minutes_to_hours_minutes(b.duration);
                            println!("{:02}:{:02} - {}", h, m, b.desc);
                            acc + b.duration
                        })
                });

            let (hours, minutes) = minutes_to_hours_minutes(total_duration_today);
            println!("--:--");
            println!("{:02}:{:02} on {} today", hours, minutes, origin.name);
        }
    }
}

fn minutes_to_hours_minutes(total_duration_today: u16) -> (u16, u16) {
    let hours = total_duration_today / 60;
    let minutes = total_duration_today % 60;
    (hours, minutes)
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
