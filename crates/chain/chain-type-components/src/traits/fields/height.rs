use cgp::prelude::*;

use crate::traits::types::height::HasHeightType;

#[cgp_component {
    provider: HeightIncrementer,
    context: Chain,
}]
pub trait CanIncrementHeight: HasHeightType + HasAsyncErrorType {
    fn increment_height(height: &Self::Height) -> Result<Self::Height, Self::Error>;
}

#[cgp_component {
    provider: HeightAdjuster,
    context: Chain,
}]
pub trait CanAdjustHeight: HasHeightType + HasAsyncErrorType {
    fn add_height(height: &Self::Height, addition: u64) -> Result<Self::Height, Self::Error>;

    fn sub_height(height: &Self::Height, subtraction: u64) -> Result<Self::Height, Self::Error>;
}
