use cgp_core::HasErrorType;
use hermes_cosmos_chain_components::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilder, ChannelOpenConfirmMessageBuilder, ChannelOpenInitMessageBuilder,
    ChannelOpenTryMessageBuilder,
};
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
    HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use crate::types::message::SolomachineMessage;

pub struct BuildCosmosToSolomachineChannelHandshakeMessage;

impl<Chain, Counterparty> ChannelOpenInitMessageBuilder<Chain, Counterparty>
    for BuildCosmosToSolomachineChannelHandshakeMessage
where
    Chain: HasInitChannelOptionsType<Counterparty>
        + HasIbcChainTypes<
            Counterparty,
            Message = SolomachineMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_init_message(
        _chain: &Chain,
        _port_id: &PortId,
        _counterparty_port_id: &PortId,
        _init_channel_options: &Chain::InitChannelOptions,
    ) -> Result<SolomachineMessage, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> ChannelOpenTryMessageBuilder<Chain, Counterparty>
    for BuildCosmosToSolomachineChannelHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = SolomachineMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType,
    Counterparty: HasChannelOpenTryPayloadType<Chain, ChannelOpenTryPayload = CosmosChannelOpenTryPayload>
        + HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
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
}

impl<Chain, Counterparty> ChannelOpenAckMessageBuilder<Chain, Counterparty>
    for BuildCosmosToSolomachineChannelHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = SolomachineMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType,
    Counterparty: HasChannelOpenAckPayloadType<Chain, ChannelOpenAckPayload = CosmosChannelOpenAckPayload>
        + HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
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
}

impl<Chain, Counterparty> ChannelOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildCosmosToSolomachineChannelHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = SolomachineMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType,
    Counterparty: HasChannelOpenConfirmPayloadType<
            Chain,
            ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
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
