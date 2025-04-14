use cgp::prelude::*;

use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::{HasRelayChains, PacketOf};

/// Encapsulates the capability of a relayer to send timeout packets over
/// unordered channels.
///
/// Timeout packets are sent from a destination chain to the source chain that
/// originated the timed out message.
///
/// When a timeout packet is sent, a response is not expected to be received.
/// This is in contrast when sending e.g. receive packets, which expect to
/// receive back a `WriteAckEvent` in response to the receive
/// packet.
#[cgp_component {
  provider: TimeoutUnorderedPacketRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayTimeoutUnorderedPacket: HasRelayChains {
    async fn relay_timeout_unordered_packet(
        &self,
        destination_height: &HeightOf<Self::DstChain>,
        packet: &PacketOf<Self>,
    ) -> Result<(), Self::Error>;
}
