use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::components::connection_handshake_message_builder::ConnectionHandshakeMessageBuilder;
use ibc_relayer_components::chain::traits::components::connection_handshake_payload_builder::ConnectionHandshakePayloadBuilder;
use ibc_relayer_components::chain::traits::types::connection::{
    HasConnectionHandshakePayloads, HasInitConnectionOptionsType,
};
use ibc_relayer_components::chain::traits::types::ibc_events::connection::{
    HasConnectionOpenInitEvent, HasConnectionOpenTryEvent,
};

use crate::one_for_all::traits::chain::{OfaChainTypes, OfaIbcChain};
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::one_for_all::types::component::OfaComponents;
use crate::std_prelude::*;

impl<Chain, Counterparty> HasConnectionHandshakePayloads<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaChainTypes,
    Counterparty: OfaChainTypes,
{
    type ConnectionOpenInitPayload = Chain::ConnectionOpenInitPayload;

    type ConnectionOpenTryPayload = Chain::ConnectionOpenTryPayload;

    type ConnectionOpenAckPayload = Chain::ConnectionOpenAckPayload;

    type ConnectionOpenConfirmPayload = Chain::ConnectionOpenConfirmPayload;
}

impl<Chain, Counterparty> HasInitConnectionOptionsType<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaChainTypes,
    Counterparty: OfaChainTypes,
{
    type InitConnectionOptions = Chain::InitConnectionOptions;
}

impl<Chain, Counterparty> HasConnectionOpenInitEvent<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    type ConnectionOpenInitEvent = Chain::ConnectionOpenInitEvent;

    fn try_extract_connection_open_init_event(
        event: Chain::Event,
    ) -> Option<Chain::ConnectionOpenInitEvent> {
        Chain::try_extract_connection_open_init_event(event)
    }

    fn connection_open_init_event_connection_id(
        event: &Chain::ConnectionOpenInitEvent,
    ) -> &Chain::ConnectionId {
        Chain::connection_open_init_event_connection_id(event)
    }
}

impl<Chain, Counterparty> HasConnectionOpenTryEvent<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    type ConnectionOpenTryEvent = Chain::ConnectionOpenTryEvent;

    fn try_extract_connection_open_try_event(
        event: Chain::Event,
    ) -> Option<Chain::ConnectionOpenTryEvent> {
        Chain::try_extract_connection_open_try_event(event)
    }

    fn connection_open_try_event_connection_id(
        event: &Chain::ConnectionOpenTryEvent,
    ) -> &Chain::ConnectionId {
        Chain::connection_open_try_event_connection_id(event)
    }
}

#[async_trait]
impl<Chain, Counterparty>
    ConnectionHandshakePayloadBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_connection_open_init_payload(
        chain: &OfaChainWrapper<Chain>,
        client_state: &Chain::ClientState,
    ) -> Result<Chain::ConnectionOpenInitPayload, Chain::Error> {
        chain
            .chain
            .build_connection_open_init_payload(client_state)
            .await
    }

    async fn build_connection_open_try_payload(
        chain: &OfaChainWrapper<Chain>,
        client_state: &Chain::ClientState,
        height: &Chain::Height,
        client_id: &Chain::ClientId,
        connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenTryPayload, Chain::Error> {
        chain
            .chain
            .build_connection_open_try_payload(client_state, height, client_id, connection_id)
            .await
    }

    async fn build_connection_open_ack_payload(
        chain: &OfaChainWrapper<Chain>,
        client_state: &Chain::ClientState,
        height: &Chain::Height,
        client_id: &Chain::ClientId,
        connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenAckPayload, Chain::Error> {
        chain
            .chain
            .build_connection_open_ack_payload(client_state, height, client_id, connection_id)
            .await
    }

    async fn build_connection_open_confirm_payload(
        chain: &OfaChainWrapper<Chain>,
        client_state: &Chain::ClientState,
        height: &Chain::Height,
        client_id: &Chain::ClientId,
        connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenConfirmPayload, Chain::Error> {
        chain
            .chain
            .build_connection_open_confirm_payload(client_state, height, client_id, connection_id)
            .await
    }
}

#[async_trait]
impl<Chain, Counterparty>
    ConnectionHandshakeMessageBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_connection_open_init_message(
        chain: &OfaChainWrapper<Chain>,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &Chain::InitConnectionOptions,
        counterparty_payload: Counterparty::ConnectionOpenInitPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        chain
            .chain
            .build_connection_open_init_message(
                client_id,
                counterparty_client_id,
                init_connection_options,
                counterparty_payload,
            )
            .await
    }

    async fn build_connection_open_try_message(
        chain: &OfaChainWrapper<Chain>,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenTryPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        chain
            .chain
            .build_connection_open_try_message(
                client_id,
                counterparty_client_id,
                counterparty_connection_id,
                counterparty_payload,
            )
            .await
    }

    async fn build_connection_open_ack_message(
        chain: &OfaChainWrapper<Chain>,
        connection_id: &Chain::ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenAckPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        chain
            .chain
            .build_connection_open_ack_message(
                connection_id,
                counterparty_connection_id,
                counterparty_payload,
            )
            .await
    }

    async fn build_connection_open_confirm_message(
        chain: &OfaChainWrapper<Chain>,
        connection_id: &Chain::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenConfirmPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        chain
            .chain
            .build_connection_open_confirm_message(connection_id, counterparty_payload)
            .await
    }
}
