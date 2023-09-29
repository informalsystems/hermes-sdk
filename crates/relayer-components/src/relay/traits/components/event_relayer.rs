use cgp_async::async_generic_trait;
use cgp_macros::derive_component;

use crate::chain::traits::types::event::HasEventType;
use crate::chain::types::aliases::{Event, Height};
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::target::ChainTarget;
use crate::std_prelude::*;

/**
   An event relayer performs relay actions based on one event at a time from
   the target chain.

   The event relayer is a general abstraction over other relayer types that
   need to be reactive to chain events. This includes the
   [packet relayer]( crate::relay::traits::components::packet_relayer::CanRelayPacket),
   but also future relayers such as connection and channel handshake relayers.
*/
#[derive_component(EventRelayerComponent, EventRelayer<Relay>)]
#[async_generic_trait]
pub trait CanRelayEvent<Target>: HasRelayChains
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
        height: &Height<Target::TargetChain>,
        event: &Event<Target::TargetChain>,
    ) -> Result<(), Self::Error>;
}
