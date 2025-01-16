use cgp::core::error::CanRaiseAsyncError;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilder;
use hermes_relayer_components::chain::traits::types::height::HasHeightFields;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::chain::traits::types::packets::ack::{
    HasAckPacketPayloadType, HasAcknowledgementType,
};
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofBytes;
use hermes_relayer_components::chain::types::payloads::packet::{
    AckPacketPayload, ReceivePacketPayload, TimeoutUnorderedPacketPayload,
};
use ibc::core::channel::types::packet::Packet;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::messages::packet::ack::CosmosAckPacketMessage;
use crate::types::messages::packet::receive::CosmosReceivePacketMessage;
use crate::types::messages::packet::timeout::CosmosTimeoutPacketMessage;

pub struct BuildCosmosPacketMessages;

impl<Chain, Counterparty> ReceivePacketMessageBuilder<Chain, Counterparty>
    for BuildCosmosPacketMessages
where
    Chain: HasMessageType + CanRaiseAsyncError<ClientError>,
    Counterparty: HasReceivePacketPayloadType<
            Chain,
            ReceivePacketPayload = ReceivePacketPayload<Counterparty>,
        > + HasHeightFields
        + HasOutgoingPacketType<Chain, OutgoingPacket = Packet>
        + HasCommitmentProofBytes,
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

        let proof_commitment =
            Counterparty::commitment_proof_bytes(&payload.proof_commitment).into();

        let message = CosmosReceivePacketMessage {
            packet: packet.clone(),
            update_height,
            proof_commitment,
        };

        Ok(message.to_cosmos_message().into())
    }
}

impl<Chain, Counterparty> AckPacketMessageBuilder<Chain, Counterparty> for BuildCosmosPacketMessages
where
    Chain: HasMessageType
        + HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet>
        + CanRaiseAsyncError<ClientError>,
    Counterparty: HasAckPacketPayloadType<Chain, AckPacketPayload = AckPacketPayload<Counterparty, Chain>>
        + HasHeightFields
        + HasCommitmentProofBytes
        + HasAcknowledgementType<Chain, Acknowledgement = Vec<u8>>,
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

        let proof_acked = Counterparty::commitment_proof_bytes(&payload.proof_ack).into();

        let message = CosmosAckPacketMessage {
            packet: packet.clone(),
            acknowledgement: payload.ack,
            update_height,
            proof_acked,
        };

        Ok(message.to_cosmos_message().into())
    }
}

impl<Chain, Counterparty> TimeoutUnorderedPacketMessageBuilder<Chain, Counterparty>
    for BuildCosmosPacketMessages
where
    Chain: HasMessageType
        + HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet>
        + CanRaiseAsyncError<ClientError>,
    Counterparty: HasTimeoutUnorderedPacketPayloadType<
            Chain,
            TimeoutUnorderedPacketPayload = TimeoutUnorderedPacketPayload<Counterparty>,
        > + HasHeightFields
        + HasCommitmentProofBytes,
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

        let proof_unreceived =
            Counterparty::commitment_proof_bytes(&payload.proof_unreceived).into();

        let message = CosmosTimeoutPacketMessage {
            next_sequence_recv: packet.seq_on_a,
            packet: packet.clone(),
            update_height,
            proof_unreceived,
        };

        Ok(message.to_cosmos_message().into())
    }
}
