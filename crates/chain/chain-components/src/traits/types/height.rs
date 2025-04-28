/*!
   Trait definition for [`HasHeightType`].
*/

pub use hermes_chain_type_components::traits::*;
use hermes_prelude::*;

#[cgp_component {
  name: HeightFieldComponent,
  provider: HeightFieldGetter,
  context: Chain,
}]
pub trait HasHeightFields: HasHeightType {
    fn revision_number(height: &Self::Height) -> u64;

    fn revision_height(height: &Self::Height) -> u64;
}

#[cgp_component {
  provider: GenesisHeightGetter,
  context: Chain,
}]
pub trait HasGenesisHeight: HasHeightType {
    fn genesis_height(&self) -> Self::Height;
}
