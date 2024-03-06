use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::ConnectionHandshakePayloadBuilder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::connection::HasConnectionHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

use crate::sovereign::types::height::RollupHeight;
use crate::sovereign::types::payloads::connection::{
    SovereignConnectionOpenAckPayload, SovereignConnectionOpenConfirmPayload,
    SovereignConnectionOpenInitPayload, SovereignConnectionOpenTryPayload,
};

pub struct BuildSovereignConnectionHandshakePayload;

impl<Chain, Counterparty> ConnectionHandshakePayloadBuilder<Chain, Counterparty>
    for BuildSovereignConnectionHandshakePayload
where
    Chain: HasConnectionHandshakePayloadTypes<
            Counterparty,
            ConnectionOpenInitPayload = SovereignConnectionOpenInitPayload,
            ConnectionOpenTryPayload = SovereignConnectionOpenTryPayload,
            ConnectionOpenAckPayload = SovereignConnectionOpenAckPayload,
            ConnectionOpenConfirmPayload = SovereignConnectionOpenConfirmPayload,
        > + HasIbcChainTypes<
            Counterparty,
            Height = RollupHeight,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasClientStateType<Counterparty>
        + HasErrorType,
{
    async fn build_connection_open_init_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
    ) -> Result<Chain::ConnectionOpenInitPayload, Chain::Error> {
        todo!()
    }

    async fn build_connection_open_try_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        _client_id: &Chain::ClientId,
        _connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenTryPayload, Chain::Error> {
        todo!()
    }

    async fn build_connection_open_ack_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        _client_id: &Chain::ClientId,
        _connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenAckPayload, Chain::Error> {
        todo!()
    }

    async fn build_connection_open_confirm_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        _client_id: &Chain::ClientId,
        _connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenConfirmPayload, Chain::Error> {
        todo!()
    }
}
