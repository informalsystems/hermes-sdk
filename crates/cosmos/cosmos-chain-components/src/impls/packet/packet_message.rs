use hermes_core::relayer_components::chain::traits::{
    AckPacketMessageBuilder, AckPacketMessageBuilderComponent, HasAckPacketPayloadType,
    HasAcknowledgementType, HasCommitmentProofBytes, HasHeightFields, HasMessageType,
    HasOutgoingPacketType, HasReceivePacketPayloadType, HasTimeoutUnorderedPacketPayloadType,
    ReceivePacketMessageBuilder, ReceivePacketMessageBuilderComponent,
    TimeoutUnorderedPacketMessageBuilder, TimeoutUnorderedPacketMessageBuilderComponent,
};
use hermes_core::relayer_components::chain::types::payloads::packet::{
    AckPacketPayload, ReceivePacketPayload, TimeoutUnorderedPacketPayload,
};
use hermes_prelude::*;
use ibc::core::channel::types::packet::Packet;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;

use crate::traits::{CosmosMessage, ToCosmosMessage};
use crate::types::{
    CosmosAckPacketMessage, CosmosReceivePacketMessage, CosmosTimeoutPacketMessage,
};

pub struct BuildCosmosPacketMessages;

#[cgp_provider(ReceivePacketMessageBuilderComponent)]
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

#[cgp_provider(AckPacketMessageBuilderComponent)]
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

#[cgp_provider(TimeoutUnorderedPacketMessageBuilderComponent)]
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
