use cgp_core::prelude::HasErrorType;

use crate::chain::traits::packet::fields::CanReadPacketFields;
use crate::chain::traits::payload_builders::ack_packet::AckPacketPayloadBuilder;
use crate::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilder;
use crate::chain::traits::payload_builders::timeout_unordered_packet::TimeoutUnorderedPacketPayloadBuilder;
use crate::chain::traits::queries::packet_acknowledgement::CanQueryPacketAcknowledgement;
use crate::chain::traits::queries::packet_commitment::CanQueryPacketCommitment;
use crate::chain::traits::queries::packet_receipt::CanQueryPacketReceipt;
use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::height::CanIncrementHeight;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::packets::ack::HasAckPacketPayloadType;
use crate::chain::traits::types::packets::ack::HasAcknowledgementType;
use crate::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use crate::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;
use crate::chain::traits::types::proof::HasCommitmentProofType;
use crate::chain::types::payloads::packet::AckPacketPayload;
use crate::chain::types::payloads::packet::ReceivePacketPayload;
use crate::chain::types::payloads::packet::TimeoutUnorderedPacketPayload;

pub struct BuildPacketPayloads;

impl<Chain, Counterparty> ReceivePacketPayloadBuilder<Chain, Counterparty> for BuildPacketPayloads
where
    Chain: HasReceivePacketPayloadType<
            Counterparty,
            ReceivePacketPayload = ReceivePacketPayload<Chain>,
        > + CanReadPacketFields<Counterparty>
        + HasClientStateType<Counterparty>
        + CanQueryPacketCommitment<Counterparty>
        + CanIncrementHeight
        + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain>,
{
    async fn build_receive_packet_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        packet: &Chain::OutgoingPacket,
    ) -> Result<Chain::ReceivePacketPayload, Chain::Error> {
        let (_, proof_commitment) = chain
            .query_packet_commitment(
                Chain::outgoing_packet_src_channel_id(packet),
                Chain::outgoing_packet_src_port(packet),
                Chain::outgoing_packet_sequence(packet),
                height,
            )
            .await?;

        let update_height = Chain::increment_height(height)?;

        // TODO: validate packet commitment

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
        + CanReadPacketFields<Counterparty>
        + HasClientStateType<Counterparty>
        + CanQueryPacketAcknowledgement<Counterparty>
        + CanIncrementHeight
        + HasCommitmentProofType
        + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain>,
    Chain::Acknowledgement: Clone,
{
    async fn build_ack_packet_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        packet: &Chain::IncomingPacket,
        ack: &Chain::Acknowledgement,
    ) -> Result<Chain::AckPacketPayload, Chain::Error> {
        let (_, proof_ack) = chain
            .query_packet_acknowledgement(
                Chain::incoming_packet_dst_channel_id(packet),
                Chain::incoming_packet_dst_port(packet),
                Chain::incoming_packet_sequence(packet),
                height,
            )
            .await?;

        let update_height = Chain::increment_height(height)?;

        // TODO: validate packet ack

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
        > + CanReadPacketFields<Counterparty>
        + HasClientStateType<Counterparty>
        + CanQueryPacketReceipt<Counterparty>
        + CanIncrementHeight
        + HasCommitmentProofType
        + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain>,
{
    async fn build_timeout_unordered_packet_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        packet: &Chain::IncomingPacket,
    ) -> Result<TimeoutUnorderedPacketPayload<Chain>, Chain::Error> {
        let (_, proof_unreceived) = chain
            .query_packet_receipt(
                Chain::incoming_packet_dst_channel_id(packet),
                Chain::incoming_packet_dst_port(packet),
                Chain::incoming_packet_sequence(packet),
                height,
            )
            .await?;

        // TODO: validate packet receipt

        let update_height = Chain::increment_height(height)?;

        let payload = TimeoutUnorderedPacketPayload {
            update_height,
            proof_unreceived,
        };

        Ok(payload)
    }
}
