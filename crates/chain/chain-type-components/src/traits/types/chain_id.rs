use core::fmt::Display;

use cgp::prelude::*;

/**
   This is implemented by a chain context to provide a `ChainId` type that
   should uniquely identify the chain.

   The relay context uses this information to identify whether an IBC packet
   corresponds to a given chain, based on the chain ID information that is
   queried from a channel ID.
*/
#[cgp_type {
    provider: ChainIdTypeProvider,
    context: Chain,
}]
pub trait HasChainIdType: Async {
    /**
    The ID of a chain, which should implement [`Eq`] to differentiate chain
    ID of two chains with the same type.
    */
    type ChainId: Eq + Display + Async;
}
