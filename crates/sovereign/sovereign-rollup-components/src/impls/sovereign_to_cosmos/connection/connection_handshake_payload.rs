use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    ConnectionOpenAckPayloadBuilder, ConnectionOpenConfirmPayloadBuilder,
    ConnectionOpenInitPayloadBuilder, ConnectionOpenTryPayloadBuilder,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

use crate::types::height::RollupHeight;
use crate::types::payloads::connection::{
    SovereignConnectionOpenAckRollupPayload, SovereignConnectionOpenConfirmRollupPayload,
    SovereignConnectionOpenInitRollupPayload, SovereignConnectionOpenTryRollupPayload,
};

pub struct BuildSovereignConnectionHandshakePayload;

impl<Chain, Counterparty> ConnectionOpenInitPayloadBuilder<Chain, Counterparty>
    for BuildSovereignConnectionHandshakePayload
where
    Chain: HasConnectionOpenInitPayloadType<
            Counterparty,
            ConnectionOpenInitPayload = SovereignConnectionOpenInitRollupPayload,
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
}

impl<Chain, Counterparty> ConnectionOpenTryPayloadBuilder<Chain, Counterparty>
    for BuildSovereignConnectionHandshakePayload
where
    Chain: HasConnectionOpenTryPayloadType<
            Counterparty,
            ConnectionOpenTryPayload = SovereignConnectionOpenTryRollupPayload,
        > + HasIbcChainTypes<
            Counterparty,
            Height = RollupHeight,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasClientStateType<Counterparty>
        + HasErrorType,
{
    async fn build_connection_open_try_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        _client_id: &Chain::ClientId,
        _connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenTryPayload, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> ConnectionOpenAckPayloadBuilder<Chain, Counterparty>
    for BuildSovereignConnectionHandshakePayload
where
    Chain: HasConnectionOpenAckPayloadType<
            Counterparty,
            ConnectionOpenAckPayload = SovereignConnectionOpenAckRollupPayload,
        > + HasIbcChainTypes<
            Counterparty,
            Height = RollupHeight,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasClientStateType<Counterparty>
        + HasErrorType,
{
    async fn build_connection_open_ack_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        _client_id: &Chain::ClientId,
        _connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenAckPayload, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> ConnectionOpenConfirmPayloadBuilder<Chain, Counterparty>
    for BuildSovereignConnectionHandshakePayload
where
    Chain: HasConnectionOpenConfirmPayloadType<
            Counterparty,
            ConnectionOpenConfirmPayload = SovereignConnectionOpenConfirmRollupPayload,
        > + HasIbcChainTypes<
            Counterparty,
            Height = RollupHeight,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasClientStateType<Counterparty>
        + HasErrorType,
{
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
