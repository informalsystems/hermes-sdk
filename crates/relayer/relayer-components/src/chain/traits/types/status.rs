use cgp::prelude::*;

use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::timestamp::HasTimestampType;

/**
   A chain context that offers a `ChainStatus` type to
   contain information about the current status of the chain.

   The `ChainStatus` type contains at minimal a `Height` field and a
   `Timestamp` field, which are accessible via the accessor methods
   `chain_status_height` and `chain_status_timestamp`.

   Using context-generic programming, the chain context may also expose
   additional fields to the chain status by introducing additional traits
   containing accessor methods. For example, one may define a
   `HasChainHealthStatus` trait to access the health status information
   from a given chain status.

   The extensible nature of the abstract `ChainStatus` type allows the
   implementation of a caching layer in the future, so that chain status
   queries can be cached without needing to know what information is contained
   inside the chain status.
*/
#[derive_component(ChainStatusTypeComponent, ProvideChainStatusType<Chain>)]
pub trait HasChainStatusType: HasHeightType + HasTimestampType {
    /**
       Contains information about the current status of the blockchain.
    */
    type ChainStatus: Async;

    /**
       Get the blockchain's current height from the chain status result.
    */
    fn chain_status_height(status: &Self::ChainStatus) -> &Self::Height;

    /**
        Get the blockchain's current timestamp from the chain status result.
    */
    fn chain_status_timestamp(status: &Self::ChainStatus) -> &Self::Timestamp;
}
