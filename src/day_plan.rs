use crate::block::Block;

pub trait DayPlan {
    fn note_id(&self) -> String;
    //    fn save(&self) -> Result<(), String>;
    fn only_original_blocks(&self) -> Vec<Block>;
    fn with_updated_blocks(self, blocks: &Vec<Block>) -> Self;
}
