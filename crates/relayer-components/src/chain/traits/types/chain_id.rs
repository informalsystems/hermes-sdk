/*!
   Trait definitions for [`HasChainIdType`] and [`HasChainId`].
*/

use core::fmt::Display;

use cgp_core::prelude::*;

/**
   This is implemented by a chain context to provide a
   [`ChainId`](Self::ChainId) type that should uniquely identify the chain.

   The relay context uses this information to identify whether an IBC packet
   corresponds to a given chain, based on the chain ID information that is
   queried from a channel ID.
*/
#[derive_component(ChainIdTypeProviderComponent, ChainIdTypeProvider<Chain>)]
pub trait HasChainIdType: Async {
    /**
       The ID of a chain, which should implement [`Eq`] to differentiate chain
       ID of two chains with the same type.
    */
    type ChainId: Eq + Display + Async;
}

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
