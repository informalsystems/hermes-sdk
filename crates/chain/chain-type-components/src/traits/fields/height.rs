use cgp::prelude::*;

use crate::traits::types::height::HasHeightType;

#[derive_component(HeightIncrementerComponent, HeightIncrementer<Chain>)]
pub trait CanIncrementHeight: HasHeightType + HasErrorType {
    fn increment_height(height: &Self::Height) -> Result<Self::Height, Self::Error>;
}
