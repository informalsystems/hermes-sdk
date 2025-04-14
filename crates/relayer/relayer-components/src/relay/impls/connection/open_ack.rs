use alloc::format;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::HasChainId;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;

use crate::chain::traits::{
    CanBuildConnectionOpenAckMessage, CanBuildConnectionOpenAckPayload, CanQueryChainHeight,
    CanQueryClientStateWithLatestHeight,
};
use crate::relay::traits::{
    CanRaiseRelayChainErrors, CanSendSingleIbcMessage, CanSendTargetUpdateClientMessage,
    ConnectionOpenAckRelayer, ConnectionOpenAckRelayerComponent, DestinationTarget,
    HasDestinationTargetChainTypes, HasRelayChains, HasRelayClientIds, HasSourceTargetChainTypes,
    MainSink, SourceTarget,
};

/**
   A base implementation of [`ConnectionOpenAckRelayer`] that relays a new connection
   at the destination chain that is in `OPEN_TRY` state, and submits it as a
   `ConnectionOpenAck` message to the destination chain.

   This implements the `ConnOpenAck` step of the IBC connection handshake protocol.

   Note that this implementation does not check that the connections at the
   source and destination chain are really in the `OPEN_TRY` state. This will be
   implemented as a separate wrapper component. (TODO)
*/
pub struct RelayConnectionOpenAck;

#[cgp_provider(ConnectionOpenAckRelayerComponent)]
impl<Relay, SrcChain, DstChain> ConnectionOpenAckRelayer<Relay> for RelayConnectionOpenAck
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + HasRelayClientIds
        + CanSendTargetUpdateClientMessage<DestinationTarget>
        + CanSendSingleIbcMessage<MainSink, SourceTarget>
        + CanLog<LevelInfo>
        + CanRaiseRelayChainErrors,
    SrcChain: CanBuildConnectionOpenAckMessage<DstChain>
        + CanQueryClientStateWithLatestHeight<DstChain>
        + HasChainId,
    DstChain: CanQueryChainHeight + CanBuildConnectionOpenAckPayload<SrcChain>,
    DstChain::ConnectionId: Clone,
{
    async fn relay_connection_open_ack(
        relay: &Relay,
        src_connection_id: &SrcChain::ConnectionId,
        dst_connection_id: &DstChain::ConnectionId,
    ) -> Result<(), Relay::Error> {
        let src_chain = relay.src_chain();
        let dst_chain = relay.dst_chain();

        let src_client_id = relay.src_client_id();
        let dst_client_id = relay.dst_client_id();

        relay
            .log(
                &format!(
                    "Starting ICS03 ConnectionOpenAck on chain `{}` for clients `{src_client_id}` and `{dst_client_id}`",
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
            .build_connection_open_ack_payload(
                &dst_client_state,
                &dst_proof_height,
                dst_client_id,
                dst_connection_id,
            )
            .await
            .map_err(Relay::raise_error)?;

        let open_ack_message = src_chain
            .build_connection_open_ack_message(
                src_connection_id,
                dst_connection_id,
                open_ack_payload,
            )
            .await
            .map_err(Relay::raise_error)?;

        relay.send_message(SourceTarget, open_ack_message).await?;

        relay
            .log(
                &format!(
                    "Successfully completed ICS03 ConnectionOpenAck on chain {} with ConnectionId `{src_connection_id}` for clients `{src_client_id}` and `{dst_client_id}`",
                    src_chain.chain_id()
                ),
                &LevelInfo,
            )
            .await;

        Ok(())
    }
}
