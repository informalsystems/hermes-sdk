use alloc::format;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::types::chain_id::HasChainId;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;

use crate::chain::traits::message_builders::channel_handshake::CanBuildChannelOpenConfirmMessage;
use crate::chain::traits::payload_builders::channel_handshake::CanBuildChannelOpenConfirmPayload;
use crate::chain::traits::queries::chain_status::CanQueryChainHeight;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds};
use crate::relay::traits::channel::open_confirm::{
    ChannelOpenConfirmRelayer, ChannelOpenConfirmRelayerComponent,
};
use crate::relay::traits::ibc_message_sender::{CanSendSingleIbcMessage, MainSink};
use crate::relay::traits::target::{DestinationTarget, HasDestinationTargetChainTypes};
use crate::relay::types::aliases::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};

/**
   A base implementation of [`ChannelOpenConfirmRelayer`] that relays a new channel
   at the source chain that is in `OPEN` state, and submits it as a
   `ChannelOpenConfirm` message to the destination chain.

   This implements the `ChanOpenConfirm` step of the IBC channel handshake protocol.

   Note that this implementation does not check that the channel exists on
   the destination chain, that a channel exists on the source chain, and that the
   channel end at the source chain is really in the `OPEN` state. This will be implemented
   as a separate wrapper component. (TODO)
*/
pub struct RelayChannelOpenConfirm;

#[cgp_provider(ChannelOpenConfirmRelayerComponent)]
impl<Relay, SrcChain, DstChain> ChannelOpenConfirmRelayer<Relay> for RelayChannelOpenConfirm
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasDestinationTargetChainTypes
        + HasRelayClientIds
        + CanSendSingleIbcMessage<MainSink, DestinationTarget>
        + CanLog<LevelInfo>
        + CanRaiseRelayChainErrors,
    SrcChain: CanQueryChainHeight + CanBuildChannelOpenConfirmPayload<DstChain>,
    DstChain: CanQueryClientStateWithLatestHeight<SrcChain>
        + CanBuildChannelOpenConfirmMessage<SrcChain>
        + HasChainId,
{
    async fn relay_channel_open_confirm(
        relay: &Relay,
        dst_port_id: &DstPortId<Relay>,
        dst_channel_id: &DstChannelId<Relay>,
        src_port_id: &SrcPortId<Relay>,
        src_channel_id: &SrcChannelId<Relay>,
    ) -> Result<(), Relay::Error> {
        let src_chain = relay.src_chain();
        let dst_chain = relay.dst_chain();

        relay
            .log(
                &format!(
                    "Starting ICS04 ChannelOpenConfirm on chain `{}`",
                    dst_chain.chain_id()
                ),
                &LevelInfo,
            )
            .await;

        let src_proof_height = src_chain
            .query_chain_height()
            .await
            .map_err(Relay::raise_error)?;

        let src_client_state = dst_chain
            .query_client_state_with_latest_height(PhantomData, relay.dst_client_id())
            .await
            .map_err(Relay::raise_error)?;

        let open_confirm_payload = src_chain
            .build_channel_open_confirm_payload(
                &src_client_state,
                &src_proof_height,
                src_port_id,
                src_channel_id,
            )
            .await
            .map_err(Relay::raise_error)?;

        let open_confirm_message = dst_chain
            .build_channel_open_confirm_message(dst_port_id, dst_channel_id, open_confirm_payload)
            .await
            .map_err(Relay::raise_error)?;

        relay
            .send_message(DestinationTarget, open_confirm_message)
            .await?;

        relay
            .log(
                &format!(
                    "Successfully completed ICS04 ChannelOpenConfirm on chain {} with ChannelId `{dst_channel_id}` and PortId `{dst_port_id}`",
                    dst_chain.chain_id()
                ),
                &LevelInfo,
            )
            .await;

        Ok(())
    }
}
