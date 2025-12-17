use std::fs;

mod block;
mod day_plan;

fn main() {
    let content = fs::read_to_string("/Users/cmw/git/criptonotes/brain/journal/2025/2025-12-11.md")
        .expect("Could not read the source file.");

    let plan = day_plan::DayPlan::from_markdown(&content, "Nubank");

    for b in plan.only_original_blocks().blocks {
        println!("{}", b.desc);
    }
}
