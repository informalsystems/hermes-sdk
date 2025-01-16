use cgp::prelude::*;

use crate::traits::types::height::HasHeightType;

#[cgp_component {
  provider: HeightIncrementer,
  context: Chain,
}]
pub trait CanIncrementHeight: HasHeightType + HasAsyncErrorType {
    fn increment_height(height: &Self::Height) -> Result<Self::Height, Self::Error>;
}
