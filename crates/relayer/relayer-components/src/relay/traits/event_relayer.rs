use cgp::prelude::*;
use hermes_chain_components::traits::types::height::HasHeightType;

use crate::chain::traits::types::event::HasEventType;
use crate::chain::types::aliases::{EventOf, HeightOf};
use crate::relay::traits::target::{HasTargetChainTypes, RelayTarget};

/**
   An event relayer performs relay actions based on one event at a time from
   the target chain.

   The event relayer is a general abstraction over other relayer types that
   need to be reactive to chain events. This includes the
   [packet relayer]( crate::relay::traits::packet_relayer::CanRelayPacket),
   but also future relayers such as connection and channel handshake relayers.
*/
#[cgp_component {
  provider: EventRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayEvent<Target: RelayTarget>:
    HasTargetChainTypes<Target, TargetChain: HasHeightType + HasEventType> + HasAsyncErrorType
{
    /**
       Relay a chain event which is emitted from the target chain at a given
       height.

       The chain event could be anything. If the given event is not related to
       IBC, the relayer should do nothing and return `Ok(())`.
    */
    async fn relay_chain_event(
        &self,
        height: &HeightOf<Self::TargetChain>,
        event: &EventOf<Self::TargetChain>,
    ) -> Result<(), Self::Error>;
}
