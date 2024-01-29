use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::components::channel_handshake_message_builder::ChannelHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloadTypes, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics04_channel::channel::{
    ChannelEnd, Counterparty as ChannelCounterparty, State,
};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::channel::CosmosInitChannelOptions;
use crate::types::messages::channel::open_ack::CosmosChannelOpenAckMessage;
use crate::types::messages::channel::open_confirm::CosmosChannelOpenConfirmMessage;
use crate::types::messages::channel::open_init::CosmosChannelOpenInitMessage;
use crate::types::messages::channel::open_try::CosmosChannelOpenTryMessage;
use crate::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};

pub struct BuildCosmosChannelHandshakeMessage;

impl<Chain, Counterparty> ChannelHandshakeMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessage
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
            ChannelOpenTryPayload = CosmosChannelOpenTryPayload,
            ChannelOpenAckPayload = CosmosChannelOpenAckPayload,
            ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload,
        >,
{
    async fn build_channel_open_init_message(
        _chain: &Chain,
        port_id: &Chain::PortId,
        counterparty_port_id: &Counterparty::PortId,
        init_channel_options: &Chain::InitChannelOptions,
    ) -> Result<Chain::Message, Chain::Error> {
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
        );

        let message = CosmosChannelOpenInitMessage { port_id, channel };

        Ok(message.to_cosmos_message())
    }

    async fn build_channel_open_try_message(
        _chain: &Chain,
        port_id: &Chain::PortId,
        counterparty_port_id: &Counterparty::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: Counterparty::ChannelOpenTryPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        let port_id = port_id.clone();
        let counterparty = ChannelCounterparty::new(
            counterparty_port_id.clone(),
            Some(counterparty_channel_id.clone()),
        );
        let ordering = counterparty_payload.ordering;
        let connection_hops = counterparty_payload.connection_hops.clone();
        let version = counterparty_payload.version.clone();

        let channel = ChannelEnd::new(
            State::TryOpen,
            ordering,
            counterparty,
            connection_hops,
            version.clone(),
        );

        let message = CosmosChannelOpenTryMessage {
            port_id,
            channel,
            counterparty_version: version,
            update_height: counterparty_payload.update_height,
            proof_init: counterparty_payload.proof_init,
        };

        Ok(message.to_cosmos_message())
    }

    async fn build_channel_open_ack_message(
        _chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: Counterparty::ChannelOpenAckPayload,
    ) -> Result<Chain::Message, Chain::Error> {
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
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        counterparty_payload: Counterparty::ChannelOpenConfirmPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        let message = CosmosChannelOpenConfirmMessage {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            update_height: counterparty_payload.update_height,
            proof_ack: counterparty_payload.proof_ack,
        };

        Ok(message.to_cosmos_message())
    }
}
