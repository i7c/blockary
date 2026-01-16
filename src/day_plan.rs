use chrono::NaiveDate;

use crate::block::Block;

pub trait DayPlanTrait {
    fn only_original_blocks(&self) -> Vec<Block>;
    fn with_updated_blocks(self, blocks: &Vec<Block>) -> Self;
    fn day(&self) -> Option<NaiveDate>;
}
