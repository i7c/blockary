use crate::blockary_cfg;
use crate::cmd_spent;
use crate::cmd_sync;
use chrono::Datelike;
use chrono::Duration;
use chrono::NaiveDate;
use clap::{Parser, Subcommand, ValueEnum};
use icalendar::Todo;
use std::env;
use std::fs;

#[derive(Parser)]
#[command(name = "blockary")]
#[command(about = "Synchronize time blocks across dayplans and calendars", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum, Debug)]
enum TimeRange {
    Today,
    ThisWeek,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync between local markdown dayplan files
    Sync {
        #[arg(short, long)]
        ics_file: Option<String>,
    },
    /// Shows how much time was spent on certain things
    Spent {
        /// Show the time spent for this period
        during: Option<TimeRange>,
    },
}

pub fn run() {
    let args = Cli::parse();
    let config = load_configuration();
    let today = chrono::Local::now().date_naive();

    match args.command {
        Commands::Sync { .. } => {
            cmd_sync::command(&config);
        }
        Commands::Spent { during } => match during {
            Some(TimeRange::Today) => cmd_spent::command(config, &today, &today),
            Some(TimeRange::ThisWeek) => {
                let (start, end) = get_week_bounds(&today);
                cmd_spent::command(config, &start, &end);
            }
            _ => println!("Provide a valid period to report time spent!"),
        },
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

fn get_week_bounds(date: &NaiveDate) -> (NaiveDate, NaiveDate) {
    // .weekday().number_from_monday() returns 1 for Mon, 7 for Sun
    // Subtracting (1-indexed value - 1) gives us the distance back to Monday
    let days_from_monday = date.weekday().number_from_monday() - 1;
    let start_of_week = *date - Duration::days(days_from_monday as i64);
    let end_of_week = start_of_week + Duration::days(6);

    (start_of_week, end_of_week)
}
