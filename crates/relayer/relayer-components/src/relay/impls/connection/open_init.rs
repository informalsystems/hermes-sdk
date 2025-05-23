use alloc::format;
use core::fmt::Debug;
use core::marker::PhantomData;

use hermes_chain_components::traits::{CanExtractFromMessageResponse, HasChainId};
use hermes_logging_components::traits::CanLog;
use hermes_logging_components::types::LevelInfo;
use hermes_prelude::*;

use crate::chain::traits::{
    CanBuildConnectionOpenInitMessage, CanBuildConnectionOpenInitPayload,
    CanQueryClientStateWithLatestHeight, CanSendSingleMessage, HasConnectionOpenInitEvent,
    HasInitConnectionOptionsType,
};
use crate::relay::traits::{
    CanRaiseRelayChainErrors, ConnectionInitializer, ConnectionInitializerComponent,
    HasRelayChains, HasRelayClientIds,
};

/**
   A base implementation for [`ConnectionInitializer`] which submits a
   `ConnectionOpenInit` message to the source chain.

   This implements the `ConnInit` step in the IBC connection handshake protocol.
*/
pub struct InitializeConnection;

pub struct MissingConnectionInitEventError<'a, Relay> {
    pub relay: &'a Relay,
}

#[cgp_provider(ConnectionInitializerComponent)]
impl<Relay, SrcChain, DstChain> ConnectionInitializer<Relay> for InitializeConnection
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasRelayClientIds
        + for<'a> CanRaiseAsyncError<MissingConnectionInitEventError<'a, Relay>>
        + CanLog<LevelInfo>
        + CanRaiseRelayChainErrors,
    SrcChain: CanSendSingleMessage
        + HasInitConnectionOptionsType<DstChain>
        + CanBuildConnectionOpenInitMessage<DstChain>
        + CanQueryClientStateWithLatestHeight<DstChain>
        + HasConnectionOpenInitEvent<DstChain>
        + CanExtractFromMessageResponse<SrcChain::ConnectionOpenInitEvent>
        + HasChainId,
    DstChain: CanBuildConnectionOpenInitPayload<SrcChain>,
    SrcChain::ConnectionId: Clone,
{
    async fn init_connection(
        relay: &Relay,
        init_connection_options: &SrcChain::InitConnectionOptions,
    ) -> Result<SrcChain::ConnectionId, Relay::Error> {
        let src_chain = relay.src_chain();
        let dst_chain = relay.dst_chain();

        let src_client_id = relay.src_client_id();
        let dst_client_id = relay.dst_client_id();

        relay
            .log(
                &format!(
                    "Starting ICS03 ConnectionOpenInit on chain `{}` for clients `{src_client_id}` and `{dst_client_id}`",
                    src_chain.chain_id()
                ),
                &LevelInfo,
            )
            .await;

        let dst_client_state = src_chain
            .query_client_state_with_latest_height(PhantomData, src_client_id)
            .await
            .map_err(Relay::raise_error)?;

        let open_init_payload = dst_chain
            .build_connection_open_init_payload(&dst_client_state)
            .await
            .map_err(Relay::raise_error)?;

        let src_message = src_chain
            .build_connection_open_init_message(
                src_client_id,
                dst_client_id,
                init_connection_options,
                open_init_payload,
            )
            .await
            .map_err(Relay::raise_error)?;

        let response = src_chain
            .send_message(src_message)
            .await
            .map_err(Relay::raise_error)?;

        let open_init_event = src_chain
            .try_extract_from_message_response(PhantomData, &response)
            .ok_or_else(|| Relay::raise_error(MissingConnectionInitEventError { relay }))?;

        let src_connection_id =
            SrcChain::connection_open_init_event_connection_id(&open_init_event);

        relay
            .log(
                &format!(
                    "Successfully completed ICS03 ConnectionOpenInit on chain {} with ConnectionId `{src_connection_id}` for client `{src_client_id}` and `{dst_client_id}`",
                    src_chain.chain_id()
                ),
                &LevelInfo,
            )
            .await;

        Ok(src_connection_id.clone())
    }
}

impl<'a, Relay> Debug for MissingConnectionInitEventError<'a, Relay> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "missing connection open init event")
    }
}
