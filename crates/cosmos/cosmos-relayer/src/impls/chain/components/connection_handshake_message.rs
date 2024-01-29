use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;
use hermes_cosmos_client_components::traits::message::CosmosMessage;
use hermes_cosmos_client_components::types::connection::CosmosInitConnectionOptions;
use hermes_relayer_components::chain::traits::components::connection_handshake_message_builder::ConnectionHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::connection::HasConnectionHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

use crate::contexts::chain::CosmosChain;
use crate::types::error::Error;

pub struct DelegateCosmosConnectionHandshakeBuilder;

impl DelegateComponent<CosmosChain> for DelegateCosmosConnectionHandshakeBuilder {
    type Delegate = BuildCosmosConnectionHandshakeMessage;
}

#[async_trait]
impl<Counterparty, Delegate> ConnectionHandshakeMessageBuilder<CosmosChain, Counterparty>
    for DelegateCosmosConnectionHandshakeBuilder
where
    Counterparty: HasConnectionHandshakePayloadTypes<CosmosChain> + HasIbcChainTypes<CosmosChain>,
    Delegate: ConnectionHandshakeMessageBuilder<CosmosChain, Counterparty>,
    Self: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_connection_open_init_message(
        chain: &CosmosChain,
        client_id: &ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &CosmosInitConnectionOptions,
        counterparty_payload: Counterparty::ConnectionOpenInitPayload,
    ) -> Result<CosmosMessage, Error> {
        Delegate::build_connection_open_init_message(
            chain,
            client_id,
            counterparty_client_id,
            init_connection_options,
            counterparty_payload,
        )
        .await
    }

    async fn build_connection_open_try_message(
        chain: &CosmosChain,
        client_id: &ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenTryPayload,
    ) -> Result<CosmosMessage, Error> {
        Delegate::build_connection_open_try_message(
            chain,
            client_id,
            counterparty_client_id,
            counterparty_connection_id,
            counterparty_payload,
        )
        .await
    }

    async fn build_connection_open_ack_message(
        chain: &CosmosChain,
        connection_id: &ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenAckPayload,
    ) -> Result<CosmosMessage, Error> {
        Delegate::build_connection_open_ack_message(
            chain,
            connection_id,
            counterparty_connection_id,
            counterparty_payload,
        )
        .await
    }

    async fn build_connection_open_confirm_message(
        chain: &CosmosChain,
        connection_id: &ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenConfirmPayload,
    ) -> Result<CosmosMessage, Error> {
        Delegate::build_connection_open_confirm_message(chain, connection_id, counterparty_payload)
            .await
    }
}
