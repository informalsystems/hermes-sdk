use cgp_core::prelude::*;
use cgp_core::HasErrorType;
use hermes_cosmos_client_components::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};
use hermes_relayer_components::chain::traits::components::channel_handshake_message_builder::ChannelHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloads, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use crate::types::message::SolomachineMessage;

pub struct BuildCosmosToSolomachineChannelHandshakeMessage;

#[async_trait]
impl<Chain, Counterparty> ChannelHandshakeMessageBuilder<Chain, Counterparty>
    for BuildCosmosToSolomachineChannelHandshakeMessage
where
    Chain: HasInitChannelOptionsType<Counterparty>
        + HasIbcChainTypes<
            Counterparty,
            Message = SolomachineMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType,
    Counterparty: HasChannelHandshakePayloads<
            Chain,
            ChannelOpenTryPayload = CosmosChannelOpenTryPayload,
            ChannelOpenAckPayload = CosmosChannelOpenAckPayload,
            ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_init_message(
        _chain: &Chain,
        _port_id: &PortId,
        _counterparty_port_id: &PortId,
        _init_channel_options: &Chain::InitChannelOptions,
    ) -> Result<SolomachineMessage, Chain::Error> {
        todo!()
    }

    async fn build_channel_open_try_message(
        _chain: &Chain,
        _port_id: &PortId,
        _counterparty_port_id: &PortId,
        _counterparty_channel_id: &ChannelId,
        counterparty_payload: CosmosChannelOpenTryPayload,
    ) -> Result<SolomachineMessage, Chain::Error> {
        let message = SolomachineMessage::CosmosChannelOpenTry(Box::new(counterparty_payload));

        Ok(message)
    }

    async fn build_channel_open_ack_message(
        _chain: &Chain,
        _port_id: &PortId,
        _channel_id: &ChannelId,
        _counterparty_channel_id: &ChannelId,
        counterparty_payload: CosmosChannelOpenAckPayload,
    ) -> Result<SolomachineMessage, Chain::Error> {
        let message = SolomachineMessage::CosmosChannelOpenAck(Box::new(counterparty_payload));

        Ok(message)
    }

    async fn build_channel_open_confirm_message(
        _chain: &Chain,
        _port_id: &PortId,
        _channel_id: &ChannelId,
        counterparty_payload: CosmosChannelOpenConfirmPayload,
    ) -> Result<SolomachineMessage, Chain::Error> {
        let message = SolomachineMessage::CosmosChannelOpenConfirm(Box::new(counterparty_payload));

        Ok(message)
    }
}
