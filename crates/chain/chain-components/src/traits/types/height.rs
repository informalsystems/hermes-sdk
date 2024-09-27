/*!
   Trait definition for [`HasHeightType`].
*/

use cgp::prelude::*;

pub use hermes_chain_type_components::traits::types::height::*;

#[derive_component(HeightFieldComponent, HeightFieldGetter<Chain>)]
pub trait HasHeightFields: HasHeightType {
    fn revision_number(height: &Self::Height) -> u64;

    fn revision_height(height: &Self::Height) -> u64;
}

#[derive_component(HeightIncrementerComponent, HeightIncrementer<Chain>)]
pub trait CanIncrementHeight: HasHeightType + HasErrorType {
    fn increment_height(height: &Self::Height) -> Result<Self::Height, Self::Error>;
}

#[derive_component(GenesisHeightGetterComponent, GenesisHeightGetter<Chain>)]
pub trait HasGenesisHeight: HasHeightType {
    fn genesis_height(&self) -> Self::Height;
}
