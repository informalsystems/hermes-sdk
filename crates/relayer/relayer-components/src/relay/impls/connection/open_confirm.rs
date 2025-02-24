use alloc::format;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::types::chain_id::HasChainId;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;

use crate::chain::traits::message_builders::connection_handshake::CanBuildConnectionOpenConfirmMessage;
use crate::chain::traits::payload_builders::connection_handshake::CanBuildConnectionOpenConfirmPayload;
use crate::chain::traits::queries::chain_status::CanQueryChainHeight;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds};
use crate::relay::traits::connection::open_confirm::{
    ConnectionOpenConfirmRelayer, ConnectionOpenConfirmRelayerComponent,
};
use crate::relay::traits::ibc_message_sender::{CanSendSingleIbcMessage, MainSink};
use crate::relay::traits::target::{DestinationTarget, HasDestinationTargetChainTypes};
use crate::relay::traits::update_client_message_builder::CanBuildTargetUpdateClientMessage;

/**
   A base implementation of [`ConnectionOpenConfirmRelayer`] that relays a new connection
   at the source chain that is in `OPEN` state, and submits it as a
   `ConnectionOpenConfirm` message to the destination chain.

   This implements the `ConnOpenConfirm` step of the IBC connection handshake protocol.

   Note that this implementation does not check that the connection at the source
   chain is really in the `OPEN` state, and that the connection at the destination chain
   is in the `OPEN_TRY` state. This will be implemented as a separate wrapper component. (TODO)
*/
pub struct RelayConnectionOpenConfirm;

#[cgp_provider(ConnectionOpenConfirmRelayerComponent)]
impl<Relay, SrcChain, DstChain> ConnectionOpenConfirmRelayer<Relay> for RelayConnectionOpenConfirm
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasRelayClientIds
        + HasDestinationTargetChainTypes
        + CanBuildTargetUpdateClientMessage<DestinationTarget>
        + CanSendSingleIbcMessage<MainSink, DestinationTarget>
        + HasLogger
        + CanRaiseRelayChainErrors,
    SrcChain: CanQueryChainHeight + CanBuildConnectionOpenConfirmPayload<DstChain>,
    DstChain: CanBuildConnectionOpenConfirmMessage<SrcChain>
        + CanQueryClientStateWithLatestHeight<SrcChain>
        + HasChainId,
    DstChain::ConnectionId: Clone,
    Relay::Logger: CanLog<LevelInfo>,
{
    async fn relay_connection_open_confirm(
        relay: &Relay,
        src_connection_id: &SrcChain::ConnectionId,
        dst_connection_id: &DstChain::ConnectionId,
    ) -> Result<(), Relay::Error> {
        let src_chain = relay.src_chain();
        let dst_chain = relay.dst_chain();

        let src_client_id = relay.src_client_id();
        let dst_client_id = relay.dst_client_id();

        relay
            .logger()
            .log(
                &format!(
                    "Starting ICS03 ConnectionOpenConfirm on chain `{}` for clients `{src_client_id}` and `{dst_client_id}`",
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
            .build_connection_open_confirm_payload(
                &src_client_state,
                &src_proof_height,
                src_client_id,
                src_connection_id,
            )
            .await
            .map_err(Relay::raise_error)?;

        let open_confirm_message = dst_chain
            .build_connection_open_confirm_message(dst_connection_id, open_confirm_payload)
            .await
            .map_err(Relay::raise_error)?;

        relay
            .send_message(DestinationTarget, open_confirm_message)
            .await?;

        relay
            .logger()
            .log(
                &format!(
                    "Successfully completed ICS03 ConnectionOpenConfirm on chain {} with ConnectionId `{src_connection_id}` for clients `{src_client_id}` and `{dst_client_id}`",
                    dst_chain.chain_id()
                ),
                &LevelInfo,
            )
            .await;

        Ok(())
    }
}
