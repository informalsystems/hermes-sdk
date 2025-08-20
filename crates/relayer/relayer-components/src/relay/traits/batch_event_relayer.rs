use alloc::vec::Vec;

use hermes_chain_components::traits::HasHeightType;
use hermes_prelude::*;

use crate::chain::traits::HasEventType;
use crate::chain::types::aliases::EventOf;
use crate::relay::traits::{HasTargetChainTypes, RelayTarget};

#[cgp_component {
  provider: BatchEventRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayBatchEvent<Target: RelayTarget>:
    HasTargetChainTypes<Target, TargetChain: HasHeightType + HasEventType> + HasAsyncErrorType
{
    /**
       Relay a batch of chain events which are emitted from the target chain.

       The chain events could be anything. If the given events are not related to
       IBC, the relayer should do nothing and return `Ok(())`.
    */
    async fn relay_chain_batch_events(
        &self,
        events: Vec<&EventOf<Self::TargetChain>>,
    ) -> Result<(), Self::Error>;
}
