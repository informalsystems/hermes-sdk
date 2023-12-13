use alloc::sync::Arc;
use async_trait::async_trait;
use cgp_core::DelegateComponent;
use cosmos_client_components::components::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;
use cosmos_client_components::traits::message::CosmosMessage;
use cosmos_client_components::types::connection::CosmosInitConnectionOptions;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_components::chain::traits::components::connection_handshake_message_builder::ConnectionHandshakeMessageBuilder;
use ibc_relayer_components::chain::traits::types::connection::HasConnectionHandshakePayloads;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

use crate::contexts::chain::CosmosChain;
use crate::types::error::Error;

pub struct DelegateCosmosConnectionHandshakeBuilder;

impl<Counterparty> DelegateComponent<CosmosChain<Counterparty>>
    for DelegateCosmosConnectionHandshakeBuilder
where
    Counterparty: ChainHandle,
{
    type Delegate = BuildCosmosConnectionHandshakeMessage;
}

#[async_trait]
impl<Chain, Counterparty, Delegate>
    ConnectionHandshakeMessageBuilder<CosmosChain<Chain>, Counterparty>
    for DelegateCosmosConnectionHandshakeBuilder
where
    Chain: ChainHandle,
    Counterparty:
        HasConnectionHandshakePayloads<CosmosChain<Chain>> + HasIbcChainTypes<CosmosChain<Chain>>,
    Delegate: ConnectionHandshakeMessageBuilder<CosmosChain<Chain>, Counterparty>,
    Self: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_connection_open_init_message(
        chain: &CosmosChain<Chain>,
        client_id: &ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &CosmosInitConnectionOptions,
        counterparty_payload: Counterparty::ConnectionOpenInitPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
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
        chain: &CosmosChain<Chain>,
        client_id: &ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenTryPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
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
        chain: &CosmosChain<Chain>,
        connection_id: &ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenAckPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        Delegate::build_connection_open_ack_message(
            chain,
            connection_id,
            counterparty_connection_id,
            counterparty_payload,
        )
        .await
    }

    async fn build_connection_open_confirm_message(
        chain: &CosmosChain<Chain>,
        connection_id: &ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenConfirmPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        Delegate::build_connection_open_confirm_message(chain, connection_id, counterparty_payload)
            .await
    }
}
