use core::fmt::Display;

use cgp::prelude::*;
use hermes_chain_type_components::traits::HasMessageType;
use hermes_relayer_components::chain::traits::{
    ChannelOpenAckMessageBuilder, ChannelOpenAckMessageBuilderComponent,
    ChannelOpenConfirmMessageBuilder, ChannelOpenConfirmMessageBuilderComponent,
    ChannelOpenInitMessageBuilder, ChannelOpenInitMessageBuilderComponent,
    ChannelOpenTryMessageBuilder, ChannelOpenTryMessageBuilderComponent, HasChannelEndType,
    HasChannelIdType, HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType,
    HasChannelOpenTryPayloadType, HasCommitmentProofBytes, HasConnectionIdType, HasHeightFields,
    HasInitChannelOptionsType, HasPortIdType,
};
use hermes_relayer_components::chain::types::payloads::channel::{
    ChannelOpenAckPayload, ChannelOpenConfirmPayload, ChannelOpenTryPayload,
};
use ibc::core::channel::types::channel::{ChannelEnd, State};
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc_proto::ibc::core::channel::v1::{Channel, Counterparty as ChannelCounterparty};

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::channel::CosmosInitChannelOptions;
use crate::types::messages::channel::open_ack::CosmosChannelOpenAckMessage;
use crate::types::messages::channel::open_confirm::CosmosChannelOpenConfirmMessage;
use crate::types::messages::channel::open_init::CosmosChannelOpenInitMessage;
use crate::types::messages::channel::open_try::CosmosChannelOpenTryMessage;

pub struct BuildCosmosChannelHandshakeMessage;

#[cgp_provider(ChannelOpenInitMessageBuilderComponent)]
impl<Chain, Counterparty> ChannelOpenInitMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessage
where
    Chain: HasMessageType
        + HasChannelIdType<Counterparty, ChannelId: Display>
        + HasPortIdType<Counterparty, PortId: Display>
        + HasInitChannelOptionsType<Counterparty, InitChannelOptions = CosmosInitChannelOptions>
        + HasAsyncErrorType,
    Counterparty: HasPortIdType<Chain, PortId: Display>,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_channel_open_init_message(
        _chain: &Chain,
        port_id: &Chain::PortId,
        counterparty_port_id: &Counterparty::PortId,
        init_channel_options: &CosmosInitChannelOptions,
    ) -> Result<Chain::Message, Chain::Error> {
        let ordering = init_channel_options.ordering;

        let connection_hops = init_channel_options
            .connection_hops
            .iter()
            .map(ToString::to_string)
            .collect();

        let channel_version = init_channel_options.channel_version.to_string();

        let channel = Channel {
            state: State::Init as i32,
            ordering: ordering as i32,
            counterparty: Some(ChannelCounterparty {
                port_id: counterparty_port_id.to_string(),
                channel_id: "".to_string(),
            }),
            connection_hops,
            version: channel_version,
            upgrade_sequence: 0,
        };

        let message = CosmosChannelOpenInitMessage {
            port_id: port_id.to_string(),
            channel,
        };

        Ok(message.to_cosmos_message().into())
    }
}

#[cgp_provider(ChannelOpenTryMessageBuilderComponent)]
impl<Chain, Counterparty> ChannelOpenTryMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessage
where
    Chain: HasMessageType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasConnectionIdType<Counterparty>
        + CanRaiseAsyncError<ClientError>,
    Counterparty: HasChannelIdType<Chain>
        + HasPortIdType<Chain>
        + HasChannelOpenTryPayloadType<
            Chain,
            ChannelOpenTryPayload = ChannelOpenTryPayload<Counterparty, Chain>,
        > + HasCommitmentProofBytes
        + HasChannelEndType<Chain, ChannelEnd = ChannelEnd>
        + HasHeightFields,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_channel_open_try_message(
        _chain: &Chain,
        port_id: &Chain::PortId,
        counterparty_port_id: &Counterparty::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        payload: ChannelOpenTryPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let ordering = payload.channel_end.ordering as i32;

        let connection_id = payload.counterparty_connection_id;

        let connection_hops = vec![connection_id.to_string()];

        let version = payload.channel_end.version.to_string();

        let channel = Channel {
            state: State::TryOpen as i32,
            ordering,
            counterparty: Some(ChannelCounterparty {
                port_id: counterparty_port_id.to_string(),
                channel_id: counterparty_channel_id.to_string(),
            }),
            connection_hops,
            version: version.clone(),
            upgrade_sequence: 0,
        };

        let update_height = Height::new(
            Counterparty::revision_number(&payload.update_height),
            Counterparty::revision_height(&payload.update_height),
        )
        .map_err(Chain::raise_error)?;

        let proof_init = Counterparty::commitment_proof_bytes(&payload.proof_init).into();

        let message = CosmosChannelOpenTryMessage {
            port_id: port_id.to_string(),
            channel,
            counterparty_version: version.to_string(),
            update_height,
            proof_init,
        };

        Ok(message.to_cosmos_message().into())
    }
}

#[cgp_provider(ChannelOpenAckMessageBuilderComponent)]
impl<Chain, Counterparty> ChannelOpenAckMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessage
where
    Chain: HasMessageType
        + HasChannelIdType<Counterparty, ChannelId: Display>
        + HasPortIdType<Counterparty, PortId: Display>
        + CanRaiseAsyncError<ClientError>,
    Counterparty: HasChannelIdType<Chain, ChannelId: Display>
        + HasPortIdType<Chain, PortId: Display>
        + HasChannelOpenAckPayloadType<
            Chain,
            ChannelOpenAckPayload = ChannelOpenAckPayload<Counterparty, Chain>,
        > + HasChannelEndType<Chain, ChannelEnd = ChannelEnd>
        + HasCommitmentProofBytes
        + HasHeightFields,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_channel_open_ack_message(
        _chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        counterparty_channel_id: &Counterparty::ChannelId,
        payload: ChannelOpenAckPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height = Height::new(
            Counterparty::revision_number(&payload.update_height),
            Counterparty::revision_height(&payload.update_height),
        )
        .map_err(Chain::raise_error)?;

        let proof_try = Counterparty::commitment_proof_bytes(&payload.proof_try).into();

        let message = CosmosChannelOpenAckMessage {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            counterparty_channel_id: counterparty_channel_id.to_string(),
            counterparty_version: payload.channel_end.version.to_string(),
            update_height,
            proof_try,
        };

        Ok(message.to_cosmos_message().into())
    }
}

#[cgp_provider(ChannelOpenConfirmMessageBuilderComponent)]
impl<Chain, Counterparty> ChannelOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessage
where
    Chain: HasMessageType
        + HasChannelIdType<Counterparty, ChannelId: Display>
        + HasPortIdType<Counterparty, PortId: Display>
        + CanRaiseAsyncError<ClientError>,
    Counterparty: HasChannelOpenConfirmPayloadType<
            Chain,
            ChannelOpenConfirmPayload = ChannelOpenConfirmPayload<Counterparty>,
        > + HasCommitmentProofBytes
        + HasHeightFields,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_channel_open_confirm_message(
        _chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        payload: ChannelOpenConfirmPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height = Height::new(
            Counterparty::revision_number(&payload.update_height),
            Counterparty::revision_height(&payload.update_height),
        )
        .map_err(Chain::raise_error)?;

        let proof_ack = Counterparty::commitment_proof_bytes(&payload.proof_ack).into();

        let message = CosmosChannelOpenConfirmMessage {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            update_height,
            proof_ack,
        };

        Ok(message.to_cosmos_message().into())
    }
}
