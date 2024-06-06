use cgp_core::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::encoder::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilder, ChannelOpenConfirmMessageBuilder, ChannelOpenInitMessageBuilder,
    ChannelOpenTryMessageBuilder,
};
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelEndType, HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType,
    HasChannelOpenTryPayloadType, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightFields;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::{
    HasCommitmentProofType, ViaCommitmentProof,
};
use hermes_relayer_components::chain::types::payloads::channel::{
    ChannelOpenAckPayload, ChannelOpenConfirmPayload, ChannelOpenTryPayload,
};
use ibc::core::channel::types::channel::{ChannelEnd, State};
use ibc_proto::ibc::core::channel::v1::{Channel, Counterparty as ChannelCounterparty};
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::Height;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::channel::CosmosInitChannelOptions;
use crate::types::messages::channel::open_ack::CosmosChannelOpenAckMessage;
use crate::types::messages::channel::open_confirm::CosmosChannelOpenConfirmMessage;
use crate::types::messages::channel::open_init::CosmosChannelOpenInitMessage;
use crate::types::messages::channel::open_try::CosmosChannelOpenTryMessage;

pub struct BuildCosmosChannelHandshakeMessage;

impl<Chain, Counterparty> ChannelOpenInitMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessage
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + HasInitChannelOptionsType<Counterparty, InitChannelOptions = CosmosInitChannelOptions>
        + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_channel_open_init_message(
        _chain: &Chain,
        port_id: &Chain::PortId,
        counterparty_port_id: &Counterparty::PortId,
        init_channel_options: &CosmosInitChannelOptions,
    ) -> Result<Chain::Message, Chain::Error> {
        let port_id = port_id.clone();
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

impl<Chain, Counterparty, CounterpartyEncoding> ChannelOpenTryMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessage
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<CounterpartyEncoding::Error>,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>
        + HasChannelOpenTryPayloadType<
            Chain,
            ChannelOpenTryPayload = ChannelOpenTryPayload<Counterparty, Chain>,
        > + HasDefaultEncoding<Encoding = CounterpartyEncoding>
        + HasCommitmentProofType
        + HasChannelEndType<Chain, ChannelEnd = ChannelEnd>
        + HasHeightFields,
    CounterpartyEncoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<ViaCommitmentProof, Counterparty::CommitmentProof>,
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

        let connection_hops = payload
            .channel_end
            .connection_hops
            .iter()
            .map(ToString::to_string)
            .collect();

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

        let counterparty_encoding = Counterparty::default_encoding();

        let proof_init = counterparty_encoding
            .encode(&payload.proof_init)
            .map_err(Chain::raise_error)?;

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

impl<Chain, Counterparty, CounterpartyEncoding> ChannelOpenAckMessageBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakeMessage
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<CounterpartyEncoding::Error>,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>
        + HasChannelOpenAckPayloadType<
            Chain,
            ChannelOpenAckPayload = ChannelOpenAckPayload<Counterparty, Chain>,
        > + HasChannelEndType<Chain, ChannelEnd = ChannelEnd>
        + HasDefaultEncoding<Encoding = CounterpartyEncoding>
        + HasCommitmentProofType
        + HasHeightFields,
    CounterpartyEncoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<ViaCommitmentProof, Counterparty::CommitmentProof>,
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

        let counterparty_encoding = Counterparty::default_encoding();

        let proof_try = counterparty_encoding
            .encode(&payload.proof_try)
            .map_err(Chain::raise_error)?;

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

impl<Chain, Counterparty, CounterpartyEncoding>
    ChannelOpenConfirmMessageBuilder<Chain, Counterparty> for BuildCosmosChannelHandshakeMessage
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<CounterpartyEncoding::Error>,
    Counterparty: HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>
        + HasChannelOpenConfirmPayloadType<
            Chain,
            ChannelOpenConfirmPayload = ChannelOpenConfirmPayload<Counterparty>,
        > + HasDefaultEncoding<Encoding = CounterpartyEncoding>
        + HasCommitmentProofType
        + HasHeightFields,
    CounterpartyEncoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<ViaCommitmentProof, Counterparty::CommitmentProof>,
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

        let counterparty_encoding = Counterparty::default_encoding();

        let proof_ack = counterparty_encoding
            .encode(&payload.proof_ack)
            .map_err(Chain::raise_error)?;

        let message = CosmosChannelOpenConfirmMessage {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            update_height,
            proof_ack,
        };

        Ok(message.to_cosmos_message().into())
    }
}
