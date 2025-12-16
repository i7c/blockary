mod block;
mod day_plan;

fn main() {
    let plan = day_plan::DayPlan::from_file("/Users/cmw/git/criptonotes/brain/journal/2025/2025-12-11.md", "Nubank");

    for b in plan.blocks {
        println!("{}", b.desc);
    }
    
    
}


