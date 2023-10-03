use cgp_async::async_trait;
use cgp_macros::derive_component;

use crate::chain::types::aliases::Height;
use crate::relay::traits::chains::HasRelayChains;
use crate::std_prelude::*;

/// Encapsulates the capability of a relayer to send timeout packets over
/// unordered channels.
///
/// Timeout packets are sent from a destination chain to the source chain that
/// originated the timed out message.
///
/// When a timeout packet is sent, a response is not expected to be received.
/// This is in contrast when sending e.g. receive packets, which expect to
/// receive back a `WriteAcknowledgementEvent` in response to the receive
/// packet.
#[derive_component(TimeoutUnorderedPacketRelayerComponent, TimeoutUnorderedPacketRelayer<Relay>)]
#[async_trait]
pub trait CanRelayTimeoutUnorderedPacket: HasRelayChains {
    async fn relay_timeout_unordered_packet(
        &self,
        destination_height: &Height<Self::DstChain>,
        packet: &Self::Packet,
    ) -> Result<(), Self::Error>;
}
