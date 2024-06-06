use cgp_core::CanRaiseError;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::encoder::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilder;
use hermes_relayer_components::chain::traits::types::height::HasHeightFields;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::{
    HasAckPacketPayloadType, HasAcknowledgementType,
};
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::proof::{
    HasCommitmentProofType, ViaCommitmentProof,
};
use hermes_relayer_components::chain::types::payloads::packet::{
    AckPacketPayload, ReceivePacketPayload, TimeoutUnorderedPacketPayload,
};
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::Height;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::messages::packet::ack::CosmosAckPacketMessage;
use crate::types::messages::packet::receive::CosmosReceivePacketMessage;
use crate::types::messages::packet::timeout::CosmosTimeoutPacketMessage;

pub struct BuildCosmosPacketMessages;

impl<Chain, Counterparty, CounterpartyEncoding> ReceivePacketMessageBuilder<Chain, Counterparty>
    for BuildCosmosPacketMessages
where
    Chain: HasMessageType
        + HasIbcPacketTypes<Counterparty, IncomingPacket = Packet>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<CounterpartyEncoding::Error>,
    Counterparty: HasReceivePacketPayloadType<
            Chain,
            ReceivePacketPayload = ReceivePacketPayload<Counterparty>,
        > + HasHeightFields
        + HasDefaultEncoding<Encoding = CounterpartyEncoding>
        + HasCommitmentProofType,
    CounterpartyEncoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<ViaCommitmentProof, Counterparty::CommitmentProof>,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_receive_packet_message(
        _chain: &Chain,
        packet: &Packet,
        payload: ReceivePacketPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height = Height::new(
            Counterparty::revision_number(&payload.update_height),
            Counterparty::revision_height(&payload.update_height),
        )
        .map_err(Chain::raise_error)?;

        let counterparty_encoding = Counterparty::default_encoding();

        let proof_commitment = counterparty_encoding
            .encode(&payload.proof_commitment)
            .map_err(Chain::raise_error)?;

        let message = CosmosReceivePacketMessage {
            packet: packet.clone(),
            update_height,
            proof_commitment,
        };

        Ok(message.to_cosmos_message().into())
    }
}

impl<Chain, Counterparty, CounterpartyEncoding> AckPacketMessageBuilder<Chain, Counterparty>
    for BuildCosmosPacketMessages
where
    Chain: HasMessageType
        + HasIbcPacketTypes<Counterparty, OutgoingPacket = Packet>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<CounterpartyEncoding::Error>,
    Counterparty: HasAckPacketPayloadType<Chain, AckPacketPayload = AckPacketPayload<Counterparty, Chain>>
        + HasHeightFields
        + HasDefaultEncoding<Encoding = CounterpartyEncoding>
        + HasCommitmentProofType
        + HasAcknowledgementType<Chain, Acknowledgement = Vec<u8>>,
    CounterpartyEncoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<ViaCommitmentProof, Counterparty::CommitmentProof>,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_ack_packet_message(
        _chain: &Chain,
        packet: &Packet,
        payload: AckPacketPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height = Height::new(
            Counterparty::revision_number(&payload.update_height),
            Counterparty::revision_height(&payload.update_height),
        )
        .map_err(Chain::raise_error)?;

        let counterparty_encoding = Counterparty::default_encoding();

        let proof_acked = counterparty_encoding
            .encode(&payload.proof_ack)
            .map_err(Chain::raise_error)?;

        let message = CosmosAckPacketMessage {
            packet: packet.clone(),
            acknowledgement: payload.ack,
            update_height,
            proof_acked,
        };

        Ok(message.to_cosmos_message().into())
    }
}

impl<Chain, Counterparty, CounterpartyEncoding>
    TimeoutUnorderedPacketMessageBuilder<Chain, Counterparty> for BuildCosmosPacketMessages
where
    Chain: HasMessageType
        + HasIbcPacketTypes<Counterparty, OutgoingPacket = Packet>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<CounterpartyEncoding::Error>,
    Counterparty: HasTimeoutUnorderedPacketPayloadType<
            Chain,
            TimeoutUnorderedPacketPayload = TimeoutUnorderedPacketPayload<Counterparty>,
        > + HasHeightFields
        + HasDefaultEncoding<Encoding = CounterpartyEncoding>
        + HasCommitmentProofType,
    CounterpartyEncoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<ViaCommitmentProof, Counterparty::CommitmentProof>,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_timeout_unordered_packet_message(
        _chain: &Chain,
        packet: &Chain::OutgoingPacket,
        payload: TimeoutUnorderedPacketPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height = Height::new(
            Counterparty::revision_number(&payload.update_height),
            Counterparty::revision_height(&payload.update_height),
        )
        .map_err(Chain::raise_error)?;

        let counterparty_encoding = Counterparty::default_encoding();

        let proof_unreceived = counterparty_encoding
            .encode(&payload.proof_unreceived)
            .map_err(Chain::raise_error)?;

        let message = CosmosTimeoutPacketMessage {
            next_sequence_recv: packet.sequence,
            packet: packet.clone(),
            update_height,
            proof_unreceived,
        };

        Ok(message.to_cosmos_message().into())
    }
}
