use cgp::prelude::*;

use crate::chain::traits::types::event::HasEventType;
use crate::chain::types::aliases::{EventOf, HeightOf};
use crate::relay::traits::chains::HasRelayClientIds;
use crate::relay::traits::target::ChainTarget;

/**
   An event relayer performs relay actions based on one event at a time from
   the target chain.

   The event relayer is a general abstraction over other relayer types that
   need to be reactive to chain events. This includes the
   [packet relayer]( crate::relay::traits::packet_relayer::CanRelayPacket),
   but also future relayers such as connection and channel handshake relayers.
*/
#[derive_component(EventRelayerComponent, EventRelayer<Relay>)]
#[async_trait]
pub trait CanRelayEvent<Target>: HasRelayClientIds
where
    Target: ChainTarget<Self>,
    Target::TargetChain: HasEventType,
{
    /**
       Relay a chain event which is emitted from the target chain at a given
       height.

       The chain event could be anything. If the given event is not related to
       IBC, the relayer should do nothing and return `Ok(())`.
    */
    async fn relay_chain_event(
        &self,
        height: &HeightOf<Target::TargetChain>,
        event: &EventOf<Target::TargetChain>,
    ) -> Result<(), Self::Error>;
}
