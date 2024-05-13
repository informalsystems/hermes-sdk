use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilder, ConnectionOpenConfirmMessageBuilder,
    ConnectionOpenInitMessageBuilder, ConnectionOpenTryMessageBuilder,
};
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
    HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::connection::CosmosInitConnectionOptions;
use crate::types::messages::connection::open_ack::CosmosConnectionOpenAckMessage;
use crate::types::messages::connection::open_confirm::CosmosConnectionOpenConfirmMessage;
use crate::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use crate::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;
use crate::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};

pub struct BuildCosmosConnectionHandshakeMessage;

impl<Chain, Counterparty> ConnectionOpenInitMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasInitConnectionOptionsType<
            Counterparty,
            InitConnectionOptions = CosmosInitConnectionOptions,
        > + HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
        > + HasBlockingChainHandle,
    Counterparty: HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasConnectionOpenInitPayloadType<
            Chain,
            ConnectionOpenInitPayload = CosmosConnectionOpenInitPayload,
        >,
{
    async fn build_connection_open_init_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &Chain::InitConnectionOptions,
        counterparty_payload: CosmosConnectionOpenInitPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let client_id = client_id.clone();
        let counterparty_client_id = counterparty_client_id.clone();
        let counterparty_commitment_prefix = counterparty_payload.commitment_prefix;
        let delay_period = init_connection_options.delay_period;

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let versions = chain_handle
                    .query_compatible_versions()
                    .map_err(Chain::raise_error)?;

                let version = versions.into_iter().next().unwrap_or_default();

                let message = CosmosConnectionOpenInitMessage {
                    client_id,
                    counterparty_client_id,
                    counterparty_commitment_prefix,
                    version,
                    delay_period,
                };

                Ok(message.to_cosmos_message())
            })
            .await
    }
}

impl<Chain, Counterparty> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
        > + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasConnectionOpenTryPayloadType<
            Chain,
            ConnectionOpenTryPayload = CosmosConnectionOpenTryPayload,
        >,
{
    async fn build_connection_open_try_message(
        _chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: CosmosConnectionOpenTryPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let message = CosmosConnectionOpenTryMessage {
            client_id: client_id.clone(),
            counterparty_client_id: counterparty_client_id.clone(),
            counterparty_connection_id: counterparty_connection_id.clone(),
            counterparty_commitment_prefix: counterparty_payload.commitment_prefix,
            counterparty_versions: counterparty_payload.versions,
            delay_period: counterparty_payload.delay_period,
            client_state: counterparty_payload.client_state,
            update_height: counterparty_payload.update_height,
            proof_init: counterparty_payload.proof_init,
            proof_client: counterparty_payload.proof_client,
            proof_consensus: counterparty_payload.proof_consensus,
            proof_consensus_height: counterparty_payload.proof_consensus_height,
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
        > + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasConnectionOpenAckPayloadType<
            Chain,
            ConnectionOpenAckPayload = CosmosConnectionOpenAckPayload,
        >,
{
    async fn build_connection_open_ack_message(
        _chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: CosmosConnectionOpenAckPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let connection_id = connection_id.clone();
        let counterparty_connection_id = counterparty_connection_id.clone();

        let message = CosmosConnectionOpenAckMessage {
            connection_id,
            counterparty_connection_id,
            version: counterparty_payload.version,
            client_state: counterparty_payload.client_state,
            update_height: counterparty_payload.update_height,
            proof_try: counterparty_payload.proof_try,
            proof_client: counterparty_payload.proof_client,
            proof_consensus: counterparty_payload.proof_consensus,
            proof_consensus_height: counterparty_payload.proof_consensus_height,
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
        > + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasConnectionOpenConfirmPayloadType<
            Chain,
            ConnectionOpenConfirmPayload = CosmosConnectionOpenConfirmPayload,
        >,
{
    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_payload: CosmosConnectionOpenConfirmPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let message = CosmosConnectionOpenConfirmMessage {
            connection_id: connection_id.clone(),
            update_height: counterparty_payload.update_height,
            proof_ack: counterparty_payload.proof_ack,
        };

        Ok(message.to_cosmos_message())
    }
}
