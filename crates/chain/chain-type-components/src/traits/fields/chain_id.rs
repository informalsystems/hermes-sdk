use hermes_prelude::*;

use crate::traits::HasChainIdType;

/**
   This implements the accessor method to get a chain context's
   [chain ID](HasChainIdType::ChainId).
*/
#[cgp_getter {
   provider: ChainIdGetter,
   context: Chain,
}]
pub trait HasChainId: HasChainIdType {
    /**
       Get the ID of a chain context. A chain context is expected to always
       return the same ID. In case there is a chain upgrade, a new chain
       context should be created with the new chain ID.
    */
    fn chain_id(&self) -> &Self::ChainId;
}
