use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use hermes_prelude::*;

use crate::chain::traits::{
    CanBuildTimeoutUnorderedPacketMessage, CanBuildTimeoutUnorderedPacketPayload,
    CanQueryClientStateWithLatestHeight,
};
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::{
    CanSendSingleIbcMessage, HasRelayChains, HasSourceTargetChainTypes, HasSrcClientId, MainSink,
    PacketOf, SourceTarget, TimeoutUnorderedPacketRelayer, TimeoutUnorderedPacketRelayerComponent,
};

/// The minimal component that implements timeout packet relayer
/// capabilities. Timeout packet relayers with more capabilities can be
/// implemented on top of this base type.
pub struct BaseTimeoutUnorderedPacketRelayer;

#[cgp_provider(TimeoutUnorderedPacketRelayerComponent)]
impl<Relay> TimeoutUnorderedPacketRelayer<Relay> for BaseTimeoutUnorderedPacketRelayer
where
    Relay: HasSourceTargetChainTypes
        + HasRelayChains
        + HasSrcClientId
        + CanRaiseAsyncError<ErrorOf<Relay::SrcChain>>
        + CanRaiseAsyncError<ErrorOf<Relay::DstChain>>,
    Relay: CanSendSingleIbcMessage<MainSink, SourceTarget>,
    Relay::SrcChain: CanQueryClientStateWithLatestHeight<Relay::DstChain>
        + CanBuildTimeoutUnorderedPacketMessage<Relay::DstChain>,
    Relay::DstChain: CanBuildTimeoutUnorderedPacketPayload<Relay::SrcChain>,
{
    async fn relay_timeout_unordered_packet(
        relay: &Relay,
        destination_height: &HeightOf<Relay::DstChain>,
        packet: &PacketOf<Relay>,
    ) -> Result<(), Relay::Error> {
        let dst_client_state = relay
            .src_chain()
            .query_client_state_with_latest_height(PhantomData, relay.src_client_id())
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
