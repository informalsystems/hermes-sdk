use cgp::prelude::HasAsyncErrorType;
use hermes_chain_type_components::traits::fields::height::CanIncrementHeight;

use crate::traits::packet::fields::{
    HasPacketDstChannelId, HasPacketDstPortId, HasPacketSequence, HasPacketSrcChannelId,
    HasPacketSrcPortId,
};
use crate::traits::payload_builders::ack_packet::AckPacketPayloadBuilder;
use crate::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilder;
use crate::traits::payload_builders::timeout_unordered_packet::TimeoutUnorderedPacketPayloadBuilder;
use crate::traits::queries::packet_acknowledgement::CanQueryPacketAcknowledgement;
use crate::traits::queries::packet_commitment::CanQueryPacketCommitment;
use crate::traits::queries::packet_receipt::CanQueryPacketReceipt;
use crate::traits::types::client_state::HasClientStateType;
use crate::traits::types::packets::ack::{HasAckPacketPayloadType, HasAcknowledgementType};
use crate::traits::types::packets::receive::HasReceivePacketPayloadType;
use crate::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;
use crate::traits::types::proof::{HasCommitmentProofHeight, HasCommitmentProofType};
use crate::types::payloads::packet::{
    AckPacketPayload, ReceivePacketPayload, TimeoutUnorderedPacketPayload,
};

pub struct BuildPacketPayloads;

impl<Chain, Counterparty> ReceivePacketPayloadBuilder<Chain, Counterparty> for BuildPacketPayloads
where
    Chain: HasReceivePacketPayloadType<
            Counterparty,
            ReceivePacketPayload = ReceivePacketPayload<Chain>,
        > + HasPacketSrcChannelId<Counterparty>
        + HasPacketSrcPortId<Counterparty>
        + HasPacketSequence<Counterparty>
        + HasClientStateType<Counterparty>
        + CanQueryPacketCommitment<Counterparty>
        + HasCommitmentProofHeight
        + HasAsyncErrorType,
{
    async fn build_receive_packet_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        packet: &Chain::OutgoingPacket,
    ) -> Result<Chain::ReceivePacketPayload, Chain::Error> {
        let (_, proof_commitment) = chain
            .query_packet_commitment(
                &Chain::packet_src_channel_id(packet),
                &Chain::packet_src_port_id(packet),
                &Chain::packet_sequence(packet),
                height,
            )
            .await?;

        // TODO: validate packet commitment

        // TODO: check that all commitment proof heights are the same
        let update_height = Chain::commitment_proof_height(&proof_commitment).clone();

        let payload = ReceivePacketPayload {
            update_height,
            proof_commitment,
        };

        Ok(payload)
    }
}

impl<Chain, Counterparty> AckPacketPayloadBuilder<Chain, Counterparty> for BuildPacketPayloads
where
    Chain: HasAckPacketPayloadType<
            Counterparty,
            AckPacketPayload = AckPacketPayload<Chain, Counterparty>,
        > + HasAcknowledgementType<Counterparty>
        + HasClientStateType<Counterparty>
        + CanQueryPacketAcknowledgement<Counterparty>
        + CanIncrementHeight
        + HasCommitmentProofHeight
        + HasAsyncErrorType,
    Counterparty:
        HasPacketDstChannelId<Chain> + HasPacketDstPortId<Chain> + HasPacketSequence<Chain>,
    Chain::Acknowledgement: Clone,
{
    async fn build_ack_packet_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        packet: &Counterparty::OutgoingPacket,
        ack: &Chain::Acknowledgement,
    ) -> Result<Chain::AckPacketPayload, Chain::Error> {
        let (_, proof_ack) = chain
            .query_packet_acknowledgement(
                &Counterparty::packet_dst_channel_id(packet),
                &Counterparty::packet_dst_port_id(packet),
                &Counterparty::packet_sequence(packet),
                height,
            )
            .await?;

        // TODO: validate paket ack

        // TODO: check that all commitment proof heights are the same
        let update_height = Chain::commitment_proof_height(&proof_ack).clone();

        let payload = AckPacketPayload {
            ack: ack.clone(),
            update_height,
            proof_ack,
        };

        Ok(payload)
    }
}

impl<Chain, Counterparty> TimeoutUnorderedPacketPayloadBuilder<Chain, Counterparty>
    for BuildPacketPayloads
where
    Chain: HasTimeoutUnorderedPacketPayloadType<
            Counterparty,
            TimeoutUnorderedPacketPayload = TimeoutUnorderedPacketPayload<Chain>,
        > + HasClientStateType<Counterparty>
        + CanQueryPacketReceipt<Counterparty>
        + HasCommitmentProofHeight
        + HasCommitmentProofType
        + HasAsyncErrorType,
    Counterparty:
        HasPacketDstChannelId<Chain> + HasPacketDstPortId<Chain> + HasPacketSequence<Chain>,
{
    async fn build_timeout_unordered_packet_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        packet: &Counterparty::OutgoingPacket,
    ) -> Result<TimeoutUnorderedPacketPayload<Chain>, Chain::Error> {
        let (_, proof_unreceived) = chain
            .query_packet_receipt(
                &Counterparty::packet_dst_channel_id(packet),
                &Counterparty::packet_dst_port_id(packet),
                &Counterparty::packet_sequence(packet),
                height,
            )
            .await?;

        // TODO: validate packet receipt

        // TODO: check that all commitment proof heights are the same
        let update_height = Chain::commitment_proof_height(&proof_unreceived).clone();

        let payload = TimeoutUnorderedPacketPayload {
            update_height,
            proof_unreceived,
        };

        Ok(payload)
    }
}
