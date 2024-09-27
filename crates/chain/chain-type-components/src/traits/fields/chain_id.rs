use cgp::prelude::*;

use crate::traits::types::chain_id::HasChainIdType;

/**
   This implements the accessor method to get a chain context's
   [chain ID](HasChainIdType::ChainId).
*/
#[derive_component(ChainIdGetterComponent, ChainIdGetter<Chain>)]
pub trait HasChainId: HasChainIdType {
    /**
       Get the ID of a chain context. A chain context is expected to always
       return the same ID. In case there is a chain upgrade, a new chain
       context should be created with the new chain ID.
    */
    fn chain_id(&self) -> &Self::ChainId;
}
