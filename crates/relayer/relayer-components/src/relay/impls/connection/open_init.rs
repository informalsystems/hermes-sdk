use core::fmt::Debug;
use core::iter::Iterator;

use cgp_core::{async_trait, CanRaiseError};

use crate::chain::traits::message_builders::connection_handshake::CanBuildConnectionHandshakeMessages;
use crate::chain::traits::payload_builders::connection_handshake::CanBuildConnectionHandshakePayloads;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::chain::traits::send_message::CanSendSingleMessage;
use crate::chain::traits::types::connection::HasInitConnectionOptionsType;
use crate::chain::traits::types::ibc_events::connection::HasConnectionOpenInitEvent;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use crate::relay::traits::connection::open_init::ConnectionInitializer;

pub struct MissingConnectionInitEventError<'a, Relay> {
    pub relay: &'a Relay,
}

impl<'a, Relay> Debug for MissingConnectionInitEventError<'a, Relay> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "missing connection init event")
    }
}

/**
   A base implementation for [`ConnectionInitializer`] which submits a
   `ConnectionOpenInit` message to the source chain.

   This implements the `ConnInit` step in the IBC connection handshake protocol.
*/
pub struct InitializeConnection;

#[async_trait]
impl<Relay, SrcChain, DstChain> ConnectionInitializer<Relay> for InitializeConnection
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + for<'a> CanRaiseError<MissingConnectionInitEventError<'a, Relay>>
        + CanRaiseRelayChainErrors,
    SrcChain: CanSendSingleMessage
        + HasInitConnectionOptionsType<DstChain>
        + CanBuildConnectionHandshakeMessages<DstChain>
        + CanQueryClientStateWithLatestHeight<DstChain>
        + HasConnectionOpenInitEvent<DstChain>,
    DstChain: CanBuildConnectionHandshakePayloads<SrcChain>,
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

        let dst_client_state = src_chain
            .query_client_state_with_latest_height(src_client_id)
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

        let events = src_chain
            .send_message(src_message)
            .await
            .map_err(Relay::raise_error)?;

        let open_init_event = events
            .into_iter()
            .find_map(|event| SrcChain::try_extract_connection_open_init_event(event))
            .ok_or_else(|| Relay::raise_error(MissingConnectionInitEventError { relay }))?;

        let src_connection_id =
            SrcChain::connection_open_init_event_connection_id(&open_init_event);

        Ok(src_connection_id.clone())
    }
}
