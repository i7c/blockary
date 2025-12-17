use std::fs;

mod block;
mod day_plan;
mod markdown_access;

fn main() {
    let work_content = fs::read_to_string("/Users/cmw/htmp/brain/2025-12-11.md")
        .expect("Could not read the source file.");

    let pers_content = fs::read_to_string("/Users/cmw/htmp/cerebro/2025-12-11.md")
        .expect("Could not read the source file.");

    let work_plan = day_plan::DayPlan::from_markdown(&work_content, "Nubank");
    let pers_plan = day_plan::DayPlan::from_markdown(&pers_content, "Personal");

    let merged = work_plan
        .only_original_blocks()
        .merge(pers_plan.only_original_blocks());

    for b in merged.blocks {
        println!("{} ({}) {}", b.period, b.origin, b.desc);
    }
}
