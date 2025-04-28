use alloc::format;
use core::marker::PhantomData;

use hermes_chain_components::traits::HasChainId;
use hermes_logging_components::traits::CanLog;
use hermes_logging_components::types::LevelInfo;
use hermes_prelude::*;

use crate::chain::traits::{
    CanBuildChannelOpenAckMessage, CanBuildChannelOpenAckPayload, CanQueryChainHeight,
    CanQueryClientStateWithLatestHeight,
};
use crate::relay::traits::{
    CanRaiseRelayChainErrors, CanSendSingleIbcMessage, ChannelOpenAckRelayer,
    ChannelOpenAckRelayerComponent, HasRelayChains, HasRelayClientIds, HasSourceTargetChainTypes,
    MainSink, SourceTarget,
};
use crate::relay::types::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};

/**
   A base implementation of [`ChannelOpenAckRelayer`] that relays a new channel
   at the destination chain that is in `OPEN_TRY` state, and submits it as a
   `ChannelOpenAck` message to the destination chain.

   This implements the `ChanOpenAck` step of the IBC channel handshake protocol.

   Note that this implementation does not check that the channel exists on
   the destination chain. It also doesn't check that the channel end at the
   destination chain is really in the `OPEN_TRY` state. This will be implemented
   as a separate wrapper component. (TODO)
*/
pub struct RelayChannelOpenAck;

#[cgp_provider(ChannelOpenAckRelayerComponent)]
impl<Relay, SrcChain, DstChain> ChannelOpenAckRelayer<Relay> for RelayChannelOpenAck
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasSourceTargetChainTypes
        + HasRelayClientIds
        + CanSendSingleIbcMessage<MainSink, SourceTarget>
        + CanLog<LevelInfo>
        + CanRaiseRelayChainErrors,
    SrcChain: CanQueryClientStateWithLatestHeight<DstChain>
        + CanBuildChannelOpenAckMessage<DstChain>
        + HasChainId,
    DstChain: CanQueryChainHeight + CanBuildChannelOpenAckPayload<SrcChain>,
{
    async fn relay_channel_open_ack(
        relay: &Relay,
        src_port_id: &SrcPortId<Relay>,
        src_channel_id: &SrcChannelId<Relay>,
        dst_port_id: &DstPortId<Relay>,
        dst_channel_id: &DstChannelId<Relay>,
    ) -> Result<(), Relay::Error> {
        let src_chain = relay.src_chain();
        let dst_chain = relay.dst_chain();

        relay
            .log(
                &format!(
                    "Starting ICS04 ChannelOpenAck on chain `{}`",
                    src_chain.chain_id()
                ),
                &LevelInfo,
            )
            .await;

        let dst_proof_height = dst_chain
            .query_chain_height()
            .await
            .map_err(Relay::raise_error)?;

        let dst_client_state = src_chain
            .query_client_state_with_latest_height(PhantomData, relay.src_client_id())
            .await
            .map_err(Relay::raise_error)?;

        let open_ack_payload = dst_chain
            .build_channel_open_ack_payload(
                &dst_client_state,
                &dst_proof_height,
                dst_port_id,
                dst_channel_id,
            )
            .await
            .map_err(Relay::raise_error)?;

        let open_ack_message = src_chain
            .build_channel_open_ack_message(
                src_port_id,
                src_channel_id,
                dst_channel_id,
                open_ack_payload,
            )
            .await
            .map_err(Relay::raise_error)?;

        relay.send_message(SourceTarget, open_ack_message).await?;

        relay
            .log(
                &format!(
                    "Successfully completed ICS04 ChannelOpenAck on chain {} with ChannelId `{src_channel_id}` and PortId `{src_port_id}`",
                    src_chain.chain_id()
                ),
                &LevelInfo,
            )
            .await;

        Ok(())
    }
}
