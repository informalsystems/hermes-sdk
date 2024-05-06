use hermes_cosmos_chain_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_ack::CosmosConnectionOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_confirm::CosmosConnectionOpenConfirmMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;
use hermes_cosmos_chain_components::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionHandshakePayloadTypes, HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

use crate::types::message::SovereignMessage;
use crate::types::payloads::connection::SovereignInitConnectionOptions;

pub struct BuildCosmosConnectionHandshakeMessageOnSovereign;

impl<Chain, Counterparty> ConnectionHandshakeMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessageOnSovereign
where
    Chain: HasInitConnectionOptionsType<
            Counterparty,
            InitConnectionOptions = SovereignInitConnectionOptions,
        > + HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasBlockingChainHandle,
    Counterparty: HasConnectionHandshakePayloadTypes<
            Chain,
            ConnectionOpenInitPayload = CosmosConnectionOpenInitPayload,
            ConnectionOpenTryPayload = CosmosConnectionOpenTryPayload,
            ConnectionOpenAckPayload = CosmosConnectionOpenAckPayload,
            ConnectionOpenConfirmPayload = CosmosConnectionOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_init_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &Chain::InitConnectionOptions,
        counterparty_payload: CosmosConnectionOpenInitPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
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

                let cosmos_message = message.to_cosmos_message();
                let sovereign_message: SovereignMessage = cosmos_message.into();

                Ok(sovereign_message)
            })
            .await
    }

    async fn build_connection_open_try_message(
        _chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: CosmosConnectionOpenTryPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        let message = CosmosConnectionOpenTryMessage {
            client_id: client_id.clone(),
            counterparty_client_id: counterparty_client_id.clone(),
            counterparty_connection_id: counterparty_connection_id.clone(),
            counterparty_commitment_prefix: counterparty_payload.commitment_prefix.clone(),
            counterparty_versions: counterparty_payload.versions,
            delay_period: counterparty_payload.delay_period,
            client_state: counterparty_payload.client_state.into(),
            update_height: counterparty_payload.update_height,
            proof_init: counterparty_payload.proof_init,
            proof_client: counterparty_payload.proof_client,
            proof_consensus: counterparty_payload.proof_consensus,
        };

        let cosmos_message = message.to_cosmos_message();
        let sovereign_message: SovereignMessage = cosmos_message.into();

        Ok(sovereign_message)
    }

    async fn build_connection_open_ack_message(
        _chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: CosmosConnectionOpenAckPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
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

        let cosmos_message = message.to_cosmos_message();
        let sovereign_message: SovereignMessage = cosmos_message.into();

        Ok(sovereign_message)
    }

    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_payload: CosmosConnectionOpenConfirmPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        let message = CosmosConnectionOpenConfirmMessage {
            connection_id: connection_id.clone(),
            update_height: counterparty_payload.update_height,
            proof_ack: counterparty_payload.proof_ack,
        };

        let cosmos_message = message.to_cosmos_message();
        let sovereign_message: SovereignMessage = cosmos_message.into();

        Ok(sovereign_message)
    }
}
