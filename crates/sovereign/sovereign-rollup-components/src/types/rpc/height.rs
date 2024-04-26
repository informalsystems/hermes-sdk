use serde::Serialize;

use crate::types::height::RollupHeight;

#[derive(Serialize)]
pub struct HeightParam {
    pub revision_number: u64,
    pub revision_height: u64,
}

impl<'a> From<&'a RollupHeight> for HeightParam {
    fn from(height: &'a RollupHeight) -> Self {
        Self {
            revision_number: 0,
            revision_height: height.slot_number,
        }
    }
}
