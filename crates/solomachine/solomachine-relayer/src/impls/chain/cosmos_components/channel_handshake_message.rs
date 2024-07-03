use cgp_core::error::HasErrorType;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::messages::channel::open_ack::CosmosChannelOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_confirm::CosmosChannelOpenConfirmMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_init::CosmosChannelOpenInitMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_try::CosmosChannelOpenTryMessage;
use hermes_error::types::Error;
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

use crate::types::payloads::channel::{
    SolomachineChannelOpenAckPayload, SolomachineChannelOpenConfirmPayload,
    SolomachineChannelOpenTryPayload,
};

pub struct BuildSolomachineChannelHandshakeMessagesForCosmos;

impl<Chain, Counterparty> ChannelOpenInitMessageBuilder<Chain, Counterparty>
    for BuildSolomachineChannelHandshakeMessagesForCosmos
where
    Chain: HasInitChannelOptionsType<Counterparty, InitChannelOptions = CosmosInitChannelOptions>
        + HasIbcChainTypes<
            Counterparty,
            Message = CosmosMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType<Error = Error>,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_init_message(
        _chain: &Chain,
        port_id: &PortId,
        counterparty_port_id: &PortId,
        init_channel_options: &CosmosInitChannelOptions,
    ) -> Result<CosmosMessage, Error> {
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
            0.into(),
        );

        let message = CosmosChannelOpenInitMessage {
            port_id: port_id.to_string(),
            channel: channel.into(),
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ChannelOpenTryMessageBuilder<Chain, Counterparty>
    for BuildSolomachineChannelHandshakeMessagesForCosmos
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = CosmosMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType<Error = Error>,
    Counterparty: HasChannelOpenTryPayloadType<
            Chain,
            ChannelOpenTryPayload = SolomachineChannelOpenTryPayload,
        > + HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_try_message(
        _chain: &Chain,
        port_id: &PortId,
        counterparty_port_id: &PortId,
        counterparty_channel_id: &ChannelId,
        counterparty_payload: SolomachineChannelOpenTryPayload,
    ) -> Result<CosmosMessage, Error> {
        let proof_init = Vec::from(counterparty_payload.proof_init.serialize_compact());

        let counterparty = ChannelCounterparty::new(
            counterparty_port_id.clone(),
            Some(counterparty_channel_id.clone()),
        );

        let channel = ChannelEnd::new(
            State::TryOpen,
            counterparty_payload.ordering,
            counterparty,
            counterparty_payload.connection_hops,
            counterparty_payload.version.clone(),
            0.into(),
        );

        let message = CosmosChannelOpenTryMessage {
            port_id: port_id.to_string(),
            channel: channel.into(),
            counterparty_version: counterparty_payload.version.to_string(),
            update_height: counterparty_payload.update_height,
            proof_init,
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ChannelOpenAckMessageBuilder<Chain, Counterparty>
    for BuildSolomachineChannelHandshakeMessagesForCosmos
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = CosmosMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType<Error = Error>,
    Counterparty: HasChannelOpenAckPayloadType<
            Chain,
            ChannelOpenAckPayload = SolomachineChannelOpenAckPayload,
        > + HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_ack_message(
        _chain: &Chain,
        port_id: &PortId,
        channel_id: &ChannelId,
        counterparty_channel_id: &ChannelId,
        counterparty_payload: SolomachineChannelOpenAckPayload,
    ) -> Result<CosmosMessage, Error> {
        let proof_try = Vec::from(counterparty_payload.proof_try.serialize_compact());

        let message = CosmosChannelOpenAckMessage {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            counterparty_channel_id: counterparty_channel_id.to_string(),
            counterparty_version: counterparty_payload.version.to_string(),
            update_height: counterparty_payload.update_height,
            proof_try,
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ChannelOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildSolomachineChannelHandshakeMessagesForCosmos
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = CosmosMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType<Error = Error>,
    Counterparty: HasChannelOpenConfirmPayloadType<
            Chain,
            ChannelOpenConfirmPayload = SolomachineChannelOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_confirm_message(
        _chain: &Chain,
        port_id: &PortId,
        channel_id: &ChannelId,
        counterparty_payload: SolomachineChannelOpenConfirmPayload,
    ) -> Result<CosmosMessage, Error> {
        let proof_ack = Vec::from(counterparty_payload.proof_ack.serialize_compact());

        let message = CosmosChannelOpenConfirmMessage {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            update_height: counterparty_payload.update_height,
            proof_ack,
        };

        Ok(message.to_cosmos_message())
    }
}
