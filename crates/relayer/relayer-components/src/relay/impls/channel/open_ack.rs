use core::marker::PhantomData;

use cgp::prelude::*;

use crate::chain::traits::message_builders::channel_handshake::CanBuildChannelOpenAckMessage;
use crate::chain::traits::payload_builders::channel_handshake::CanBuildChannelOpenAckPayload;
use crate::chain::traits::queries::chain_status::CanQueryChainHeight;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds};
use crate::relay::traits::channel::open_ack::{
    ChannelOpenAckRelayer, ChannelOpenAckRelayerComponent,
};
use crate::relay::traits::ibc_message_sender::{CanSendSingleIbcMessage, MainSink};
use crate::relay::traits::target::{HasSourceTargetChainTypes, SourceTarget};
use crate::relay::types::aliases::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};

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
        + CanRaiseRelayChainErrors,
    SrcChain:
        CanQueryClientStateWithLatestHeight<DstChain> + CanBuildChannelOpenAckMessage<DstChain>,
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

        Ok(())
    }
}
