use cgp_core::prelude::*;
use cgp_core::HasErrorType;
use hermes_cosmos_client_components::traits::message::CosmosMessage;
use hermes_cosmos_client_components::types::channel::CosmosInitChannelOptions;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloadTypes, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use crate::sovereign::types::payloads::channel::{
    SovereignChannelOpenAckPayload, SovereignChannelOpenConfirmPayload,
    SovereignChannelOpenTryPayload,
};

pub struct BuildSovereignChannelHandshakeMessageOnCosmos;

#[async_trait]
impl<Chain, Counterparty> ChannelHandshakeMessageBuilder<Chain, Counterparty>
    for BuildSovereignChannelHandshakeMessageOnCosmos
where
    Chain: HasIbcChainTypes<
            Counterparty,
            ChannelId = ChannelId,
            PortId = PortId,
            Message = CosmosMessage,
        > + HasInitChannelOptionsType<Counterparty, InitChannelOptions = CosmosInitChannelOptions>
        + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>
        + HasChannelHandshakePayloadTypes<
            Chain,
            ChannelOpenTryPayload = SovereignChannelOpenTryPayload,
            ChannelOpenAckPayload = SovereignChannelOpenAckPayload,
            ChannelOpenConfirmPayload = SovereignChannelOpenConfirmPayload,
        >,
{
    async fn build_channel_open_init_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _counterparty_port_id: &Counterparty::PortId,
        _init_channel_options: &CosmosInitChannelOptions,
    ) -> Result<CosmosMessage, Chain::Error> {
        todo!()
    }

    async fn build_channel_open_try_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _counterparty_port_id: &Counterparty::PortId,
        _counterparty_channel_id: &Counterparty::ChannelId,
        _counterparty_payload: SovereignChannelOpenTryPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        todo!()
    }

    async fn build_channel_open_ack_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
        _counterparty_channel_id: &Counterparty::ChannelId,
        _counterparty_payload: SovereignChannelOpenAckPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        todo!()
    }

    async fn build_channel_open_confirm_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
        _counterparty_payload: SovereignChannelOpenConfirmPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        todo!()
    }
}
