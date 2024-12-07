use cgp::prelude::*;

use crate::traits::types::height::HasHeightType;

#[cgp_component {
  name: HeightIncrementerComponent,
  provider: HeightIncrementer,
  context: Chain,
}]
pub trait CanIncrementHeight: HasHeightType + HasErrorType {
    fn increment_height(height: &Self::Height) -> Result<Self::Height, Self::Error>;
}
