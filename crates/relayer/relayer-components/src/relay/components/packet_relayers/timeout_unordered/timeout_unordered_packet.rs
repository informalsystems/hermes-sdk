use cgp_core::async_trait;

use crate::chain::traits::components::client_state_querier::CanQueryClientState;
use crate::chain::traits::components::timeout_unordered_packet_message_builder::{
    CanBuildTimeoutUnorderedPacketMessage, CanBuildTimeoutUnorderedPacketPayload,
};
use crate::chain::types::aliases::Height;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::components::ibc_message_sender::{CanSendSingleIbcMessage, MainSink};
use crate::relay::traits::components::packet_relayers::timeout_unordered_packet::TimeoutUnorderedPacketRelayer;
use crate::relay::traits::target::SourceTarget;
use crate::relay::types::aliases::Packet;

/// The minimal component that implements timeout packet relayer
/// capabilities. Timeout packet relayers with more capabilities can be
/// implemented on top of this base type.
pub struct BaseTimeoutUnorderedPacketRelayer;

#[async_trait]
impl<Relay> TimeoutUnorderedPacketRelayer<Relay> for BaseTimeoutUnorderedPacketRelayer
where
    Relay: HasRelayChains,
    Relay: CanSendSingleIbcMessage<MainSink, SourceTarget>,
    Relay::SrcChain: CanQueryClientState<Relay::DstChain>
        + CanBuildTimeoutUnorderedPacketMessage<Relay::DstChain>,
    Relay::DstChain: CanBuildTimeoutUnorderedPacketPayload<Relay::SrcChain>,
{
    async fn relay_timeout_unordered_packet(
        relay: &Relay,
        destination_height: &Height<Relay::DstChain>,
        packet: &Packet<Relay>,
    ) -> Result<(), Relay::Error> {
        let dst_client_state = relay
            .src_chain()
            .query_client_state(relay.src_client_id())
            .await
            .map_err(Relay::raise_error)?;

        let payload = relay
            .dst_chain()
            .build_timeout_unordered_packet_payload(&dst_client_state, destination_height, packet)
            .await
            .map_err(Relay::raise_error)?;

        let message = relay
            .src_chain()
            .build_timeout_unordered_packet_message(packet, payload)
            .await
            .map_err(Relay::raise_error)?;

        relay.send_message(SourceTarget, message).await?;

        Ok(())
    }
}
