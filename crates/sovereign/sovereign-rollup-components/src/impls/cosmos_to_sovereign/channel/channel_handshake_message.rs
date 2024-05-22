use cgp_core::HasErrorType;

use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_ack::CosmosChannelOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_confirm::CosmosChannelOpenConfirmMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_init::CosmosChannelOpenInitMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_try::CosmosChannelOpenTryMessage;
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

use ibc_relayer_types::core::ics04_channel::channel::{
    ChannelEnd, Counterparty as ChannelCounterparty, State,
};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use crate::types::message::SovereignMessage;
use crate::types::payloads::channel::SovereignInitChannelOptions;

pub struct BuildCosmosChannelHandshakeMessageOnSovereign;

impl<Rollup, Counterparty> ChannelOpenInitMessageBuilder<Rollup, Counterparty>
    for BuildCosmosChannelHandshakeMessageOnSovereign
where
    Rollup: HasInitChannelOptionsType<Counterparty, InitChannelOptions = SovereignInitChannelOptions>
        + HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType,
    Counterparty: HasIbcChainTypes<Rollup, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_init_message(
        _rollup: &Rollup,
        port_id: &Rollup::PortId,
        counterparty_port_id: &Counterparty::PortId,
        init_channel_options: &SovereignInitChannelOptions,
    ) -> Result<SovereignMessage, Rollup::Error> {
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

        let cosmos_msg = message.to_cosmos_message();
        let sovereign_msg: SovereignMessage = cosmos_msg.into();

        Ok(sovereign_msg)
    }
}

impl<Rollup, Counterparty> ChannelOpenTryMessageBuilder<Rollup, Counterparty>
    for BuildCosmosChannelHandshakeMessageOnSovereign
where
    Rollup: HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType,
    Counterparty: HasChannelOpenTryPayloadType<Rollup, ChannelOpenTryPayload = CosmosChannelOpenTryPayload>
        + HasIbcChainTypes<Rollup, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_try_message(
        _rollup: &Rollup,
        port_id: &Rollup::PortId,
        counterparty_port_id: &Counterparty::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: CosmosChannelOpenTryPayload,
    ) -> Result<SovereignMessage, Rollup::Error> {
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

        let msg = CosmosChannelOpenTryMessage {
            port_id: port_id.to_string(),
            channel: channel.into(),
            counterparty_version: version.to_string(),
            update_height: counterparty_payload.update_height,
            proof_init: counterparty_payload.proof_init.into(),
        };

        let cosmos_msg = msg.to_cosmos_message();
        let sovereign_msg: SovereignMessage = cosmos_msg.into();

        Ok(sovereign_msg)
    }
}

impl<Rollup, Counterparty> ChannelOpenAckMessageBuilder<Rollup, Counterparty>
    for BuildCosmosChannelHandshakeMessageOnSovereign
where
    Rollup: HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType,
    Counterparty: HasChannelOpenAckPayloadType<Rollup, ChannelOpenAckPayload = CosmosChannelOpenAckPayload>
        + HasIbcChainTypes<Rollup, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_ack_message(
        _rollup: &Rollup,
        port_id: &Rollup::PortId,
        channel_id: &Rollup::ChannelId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: CosmosChannelOpenAckPayload,
    ) -> Result<SovereignMessage, Rollup::Error> {
        let msg = CosmosChannelOpenAckMessage {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            counterparty_channel_id: counterparty_channel_id.to_string(),
            counterparty_version: counterparty_payload.version.to_string(),
            update_height: counterparty_payload.update_height,
            proof_try: counterparty_payload.proof_try.into(),
        };

        let cosmos_msg = msg.to_cosmos_message();
        let sovereign_msg: SovereignMessage = cosmos_msg.into();

        Ok(sovereign_msg)
    }
}

impl<Rollup, Counterparty> ChannelOpenConfirmMessageBuilder<Rollup, Counterparty>
    for BuildCosmosChannelHandshakeMessageOnSovereign
where
    Rollup: HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType,
    Counterparty: HasChannelOpenConfirmPayloadType<
            Rollup,
            ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload,
        > + HasIbcChainTypes<Rollup, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_confirm_message(
        _rollup: &Rollup,
        port_id: &Rollup::PortId,
        channel_id: &Rollup::ChannelId,
        counterparty_payload: CosmosChannelOpenConfirmPayload,
    ) -> Result<SovereignMessage, Rollup::Error> {
        let msg = CosmosChannelOpenConfirmMessage {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            update_height: counterparty_payload.update_height,
            proof_ack: counterparty_payload.proof_ack.into(),
        };

        let cosmos_msg = msg.to_cosmos_message();
        let sovereign_msg: SovereignMessage = cosmos_msg.into();

        Ok(sovereign_msg)
    }
}
