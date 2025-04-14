use alloc::format;
use core::fmt::Debug;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::{CanExtractFromMessageResponse, HasChainId};
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;

use crate::chain::traits::{
    CanBuildConnectionOpenTryMessage, CanBuildConnectionOpenTryPayload, CanQueryChainHeight,
    CanQueryClientStateWithLatestHeight, HasConnectionOpenTryEvent,
};
use crate::chain::types::aliases::ConnectionIdOf;
use crate::relay::traits::{
    CanRaiseRelayChainErrors, CanSendSingleIbcMessage, CanSendTargetUpdateClientMessage,
    ConnectionOpenTryRelayer, ConnectionOpenTryRelayerComponent, DestinationTarget,
    HasDestinationTargetChainTypes, HasRelayChains, HasRelayClientIds, HasSourceTargetChainTypes,
    MainSink, SourceTarget,
};

/**
   A base implementation of [`ConnectionOpenTryRelayer`] that relays a new connection
   at the source chain that is in `OPEN_INIT` state, and submits it as a
   `ConnectionOpenTry` message to the destination chain.

   This implements the `ConnOpenTry` step of the IBC connection handshake protocol.

   Note that this implementation does not check that the connection at the source
   chain is really in the `OPEN_INIT` state. This will be implemented as a separate
   wrapper component. (TODO)
*/
pub struct RelayConnectionOpenTry;

pub struct MissingConnectionTryEventError<'a, Relay>
where
    Relay: HasRelayChains,
{
    pub relay: &'a Relay,
    pub src_connection_id: &'a ConnectionIdOf<Relay::SrcChain, Relay::DstChain>,
}

#[cgp_provider(ConnectionOpenTryRelayerComponent)]
impl<Relay, SrcChain, DstChain> ConnectionOpenTryRelayer<Relay> for RelayConnectionOpenTry
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + HasRelayClientIds
        + CanSendTargetUpdateClientMessage<SourceTarget>
        + CanSendSingleIbcMessage<MainSink, DestinationTarget>
        + for<'a> CanRaiseAsyncError<MissingConnectionTryEventError<'a, Relay>>
        + CanLog<LevelInfo>
        + CanRaiseRelayChainErrors,
    SrcChain: CanQueryChainHeight + CanBuildConnectionOpenTryPayload<DstChain>,
    DstChain: CanQueryClientStateWithLatestHeight<SrcChain>
        + CanBuildConnectionOpenTryMessage<SrcChain>
        + HasConnectionOpenTryEvent<SrcChain>
        + CanExtractFromMessageResponse<DstChain::ConnectionOpenTryEvent>
        + HasChainId,
    DstChain::ConnectionId: Clone,
{
    async fn relay_connection_open_try(
        relay: &Relay,
        src_connection_id: &SrcChain::ConnectionId,
    ) -> Result<DstChain::ConnectionId, Relay::Error> {
        let src_chain = relay.src_chain();
        let dst_chain = relay.dst_chain();

        let src_client_id = relay.src_client_id();
        let dst_client_id = relay.dst_client_id();

        relay
            .log(
                &format!(
                    "Starting ICS03 ConnectionOpenTry on chain `{}` for clients `{src_client_id}` and `{dst_client_id}`",
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

        let open_try_payload = src_chain
            .build_connection_open_try_payload(
                &src_client_state,
                &src_proof_height,
                src_client_id,
                src_connection_id,
            )
            .await
            .map_err(Relay::raise_error)?;

        let open_try_message = dst_chain
            .build_connection_open_try_message(
                dst_client_id,
                src_client_id,
                src_connection_id,
                open_try_payload,
            )
            .await
            .map_err(Relay::raise_error)?;

        let response = relay
            .send_message(DestinationTarget, open_try_message)
            .await?;

        let open_try_event = dst_chain
            .try_extract_from_message_response(PhantomData, &response)
            .ok_or_else(|| {
                Relay::raise_error(MissingConnectionTryEventError {
                    relay,
                    src_connection_id,
                })
            })?;

        let dst_connection_id = DstChain::connection_open_try_event_connection_id(&open_try_event);

        relay
            .log(
                &format!(
                    "Successfully completed ICS03 ConnectionOpenTry on chain {} with ConnectionId `{dst_connection_id}` for clients `{src_client_id}` and `{dst_client_id}`",
                    dst_chain.chain_id()
                ),
                &LevelInfo,
            )
            .await;

        Ok(dst_connection_id.clone())
    }
}

impl<'a, Relay> Debug for MissingConnectionTryEventError<'a, Relay>
where
    Relay: HasRelayChains,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MissingConnectionTryEventError")
            .field("src_connection_id", &self.src_connection_id)
            .finish()
    }
}
