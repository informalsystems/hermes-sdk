use cgp_core::HasErrorType;
use hermes_cosmos_chain_components::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloadTypes, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use crate::types::message::SovereignMessage;
use crate::types::payloads::channel::SovereignInitChannelOptions;

pub struct BuildCosmosChannelHandshakeMessageOnSovereign;

impl<Chain, Counterparty> ChannelHandshakeMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessageOnSovereign
where
    Chain: HasInitChannelOptionsType<Counterparty, InitChannelOptions = SovereignInitChannelOptions>
        + HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType,
    Counterparty: HasChannelHandshakePayloadTypes<
            Chain,
            ChannelOpenTryPayload = CosmosChannelOpenTryPayload,
            ChannelOpenAckPayload = CosmosChannelOpenAckPayload,
            ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_init_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _counterparty_port_id: &Counterparty::PortId,
        _init_channel_options: &SovereignInitChannelOptions,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }

    async fn build_channel_open_try_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _counterparty_port_id: &Counterparty::PortId,
        _counterparty_channel_id: &Counterparty::ChannelId,
        _counterparty_payload: CosmosChannelOpenTryPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }

    async fn build_channel_open_ack_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
        _counterparty_channel_id: &Counterparty::ChannelId,
        _counterparty_payload: CosmosChannelOpenAckPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }

    async fn build_channel_open_confirm_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
        _counterparty_payload: CosmosChannelOpenConfirmPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }
}
