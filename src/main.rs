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


    let paths = sync::day_plans_from_directory("Personal", "/Users/cmw/git/fortytwo/journal");

    for p in paths {
        println!("{}", p.note_id())
    }
}
