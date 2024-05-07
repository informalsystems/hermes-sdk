use cgp_core::HasErrorType;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_chain_components::types::messages::connection::open_ack::CosmosConnectionOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionHandshakePayloadTypes, HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics03_connection::version::Version;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

use crate::sovereign::types::payloads::connection::{
    SovereignConnectionOpenAckPayload, SovereignConnectionOpenConfirmPayload,
    SovereignConnectionOpenInitPayload, SovereignConnectionOpenTryPayload,
};

pub struct BuildSovereignConnectionHandshakeMessageOnCosmos;

impl<Chain, Counterparty> ConnectionHandshakeMessageBuilder<Chain, Counterparty>
    for BuildSovereignConnectionHandshakeMessageOnCosmos
where
    Chain: HasInitConnectionOptionsType<
            Counterparty,
            InitConnectionOptions = CosmosInitConnectionOptions,
        > + HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
        > + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasConnectionHandshakePayloadTypes<
            Chain,
            ConnectionOpenInitPayload = SovereignConnectionOpenInitPayload,
            ConnectionOpenTryPayload = SovereignConnectionOpenTryPayload,
            ConnectionOpenAckPayload = SovereignConnectionOpenAckPayload,
            ConnectionOpenConfirmPayload = SovereignConnectionOpenConfirmPayload,
        >,
{
    async fn build_connection_open_init_message(
        _chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &Chain::InitConnectionOptions,
        counterparty_payload: SovereignConnectionOpenInitPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        // TODO: Retrieve version and delay period
        let version = Version::default();
        let delay_period = init_connection_options.delay_period;

        let counterparty_commitment_prefix = counterparty_payload.commitment_prefix;

        let message = CosmosConnectionOpenInitMessage {
            client_id: client_id.clone(),
            counterparty_client_id: counterparty_client_id.clone(),
            counterparty_commitment_prefix,
            version,
            delay_period,
        };

        Ok(message.to_cosmos_message())
    }

    async fn build_connection_open_try_message(
        _chain: &Chain,
        _client_id: &Chain::ClientId,
        _counterparty_client_id: &Counterparty::ClientId,
        _counterparty_connection_id: &Counterparty::ConnectionId,
        _counterparty_payload: SovereignConnectionOpenTryPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        todo!()
    }

    async fn build_connection_open_ack_message(
        _chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: SovereignConnectionOpenAckPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let connection_id = connection_id.clone();
        let counterparty_connection_id = counterparty_connection_id.clone();

        let message = CosmosConnectionOpenAckMessage {
            connection_id,
            counterparty_connection_id,
            version: counterparty_payload.version,
            client_state: counterparty_payload.client_state.into(),
            update_height: counterparty_payload.update_height,
            proof_try: counterparty_payload.proof_try,
            proof_client: counterparty_payload.proof_client,
            proof_consensus: counterparty_payload.proof_consensus,
        };

        Ok(message.to_cosmos_message())
    }

    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        _connection_id: &Chain::ConnectionId,
        _counterparty_payload: SovereignConnectionOpenConfirmPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        todo!()
    }
}
