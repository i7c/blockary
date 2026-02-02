use crate::blockary_cfg;
use crate::cmd_spent;
use crate::cmd_sync;
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
    /// Shows how much time was spent on certain things
    Spent,
}

pub fn run() {
    let args = Cli::parse();
    let config = load_configuration();
    let today = chrono::Local::now().date_naive();

    match args.command {
        Commands::Sync { .. } => {
            cmd_sync::command(&config);
        }
        Commands::Spent => {
            cmd_spent::command(config, today);
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
