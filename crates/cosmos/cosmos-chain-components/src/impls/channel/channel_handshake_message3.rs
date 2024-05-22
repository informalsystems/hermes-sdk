use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilder, ChannelOpenConfirmMessageBuilder, ChannelOpenInitMessageBuilder,
    ChannelOpenTryMessageBuilder,
};
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
    HasInitChannelOptionsType,
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

impl<Chain, Counterparty> ChannelOpenInitMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            ChannelId = ChannelId,
            PortId = PortId,
            Message = CosmosMessage,
        > + HasInitChannelOptionsType<Counterparty, InitChannelOptions = CosmosInitChannelOptions>
        + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
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

        let message = CosmosChannelOpenInitMessage {
            port_id: port_id.to_string(),
            channel: channel.into(),
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ChannelOpenTryMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            ChannelId = ChannelId,
            PortId = PortId,
            Message = CosmosMessage,
        > + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>
        + HasChannelOpenTryPayloadType<Chain, ChannelOpenTryPayload = CosmosChannelOpenTryPayload>,
{
    async fn build_channel_open_try_message(
        _chain: &Chain,
        port_id: &Chain::PortId,
        counterparty_port_id: &Counterparty::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: CosmosChannelOpenTryPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
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
            0,
        );

        let message = CosmosChannelOpenTryMessage {
            port_id: port_id.to_string(),
            channel: channel.into(),
            counterparty_version: version.to_string(),
            update_height: counterparty_payload.update_height,
            proof_init: counterparty_payload.proof_init.into(),
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ChannelOpenAckMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            ChannelId = ChannelId,
            PortId = PortId,
            Message = CosmosMessage,
        > + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>
        + HasChannelOpenAckPayloadType<Chain, ChannelOpenAckPayload = CosmosChannelOpenAckPayload>,
{
    async fn build_channel_open_ack_message(
        _chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: CosmosChannelOpenAckPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let message = CosmosChannelOpenAckMessage {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            counterparty_channel_id: counterparty_channel_id.to_string(),
            counterparty_version: counterparty_payload.version.to_string(),
            update_height: counterparty_payload.update_height,
            proof_try: counterparty_payload.proof_try.into(),
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ChannelOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            ChannelId = ChannelId,
            PortId = PortId,
            Message = CosmosMessage,
        > + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>
        + HasChannelOpenConfirmPayloadType<
            Chain,
            ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload,
        >,
{
    async fn build_channel_open_confirm_message(
        _chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        counterparty_payload: CosmosChannelOpenConfirmPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let message = CosmosChannelOpenConfirmMessage {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            update_height: counterparty_payload.update_height,
            proof_ack: counterparty_payload.proof_ack.into(),
        };

        Ok(message.to_cosmos_message())
    }
}
