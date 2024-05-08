use cgp_core::prelude::*;
use cgp_core::HasErrorType;
use hermes_cosmos_chain_components::traits::message::CosmosMessage;
use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::messages::channel::open_ack::CosmosChannelOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_init::CosmosChannelOpenInitMessage;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloadTypes, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics04_channel::channel::ChannelEnd;
use ibc_relayer_types::core::ics04_channel::channel::Counterparty as ChannelCounterparty;
use ibc_relayer_types::core::ics04_channel::channel::State;
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
        port_id: &Chain::PortId,
        counterparty_port_id: &Counterparty::PortId,
        init_channel_options: &CosmosInitChannelOptions,
    ) -> Result<CosmosMessage, Chain::Error> {
        let port_id = port_id.clone();
        let ordering = init_channel_options.ordering;
        let connection_hops = init_channel_options.connection_hops.clone();
        let channel_version = init_channel_options.channel_version.clone();

        let counterparty = ChannelCounterparty::new(counterparty_port_id.clone(), None);

        let channel = ChannelEnd::new(
            State::Init,
            ordering,
            counterparty,
            connection_hops,
            channel_version,
            0,
        );

        let message = CosmosChannelOpenInitMessage { port_id, channel };

        Ok(message.to_cosmos_message())
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
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: SovereignChannelOpenAckPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let message = CosmosChannelOpenAckMessage {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            counterparty_channel_id: counterparty_channel_id.clone(),
            counterparty_version: counterparty_payload.version,
            update_height: counterparty_payload.update_height,
            proof_try: counterparty_payload.proof_try,
        };

        Ok(message.to_cosmos_message())
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
