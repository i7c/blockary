use crate::{
    block::Block,
    day_plan::{self, DayPlan},
    markdown_access,
};

pub fn day_plan_from_daily_file_md(
    markdown_content: &str,
    origin: &str,
    abs_path: &str,
    base_dir: &str,
) -> DayPlan {
    let block_strings = markdown_access::read_items_under_section(markdown_content, "Time Blocks");
    let blocks = block_strings
        .iter()
        .map(|bs| Block::parse_block_string(origin, bs).expect(""))
        .collect();

    DayPlan {
        origin: origin.to_string(),
        blocks: blocks,
        source: day_plan::Source::ObsMarkDown {
            abs_path: abs_path.to_string(),
            base_dir: base_dir.to_string(),
        },
        day: None,
    }
}
