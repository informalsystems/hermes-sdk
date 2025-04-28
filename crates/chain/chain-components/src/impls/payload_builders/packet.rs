use core::fmt::Debug;

use hermes_chain_type_components::traits::{
    CanIncrementHeight, HasHeightType, HasOutgoingPacketType,
};
use hermes_prelude::*;

use crate::traits::{
    AckPacketPayloadBuilder, AckPacketPayloadBuilderComponent, CanQueryPacketAckCommitment,
    CanQueryPacketCommitment, CanQueryPacketReceipt, HasAckPacketPayloadType,
    HasAcknowledgementType, HasClientStateType, HasCommitmentProofHeight, HasCommitmentProofType,
    HasPacketDstChannelId, HasPacketDstPortId, HasPacketSequence, HasPacketSrcChannelId,
    HasPacketSrcPortId, HasReceivePacketPayloadType, HasTimeoutUnorderedPacketPayloadType,
    ReceivePacketPayloadBuilder, ReceivePacketPayloadBuilderComponent,
    TimeoutUnorderedPacketPayloadBuilder, TimeoutUnorderedPacketPayloadBuilderComponent,
};
use crate::types::payloads::packet::{
    AckPacketPayload, ReceivePacketPayload, TimeoutUnorderedPacketPayload,
};

pub struct BuildPacketPayloads;

#[cgp_provider(ReceivePacketPayloadBuilderComponent)]
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

#[cgp_provider(AckPacketPayloadBuilderComponent)]
impl<Chain, Counterparty> AckPacketPayloadBuilder<Chain, Counterparty> for BuildPacketPayloads
where
    Chain: HasAckPacketPayloadType<
            Counterparty,
            AckPacketPayload = AckPacketPayload<Chain, Counterparty>,
        > + HasAcknowledgementType<Counterparty>
        + HasClientStateType<Counterparty>
        + CanQueryPacketAckCommitment<Counterparty>
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
            .query_packet_ack_commitment_with_proof(
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

#[cgp_provider(TimeoutUnorderedPacketPayloadBuilderComponent)]
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
        + for<'a> CanRaiseAsyncError<InvalidTimeoutReceipt<'a, Chain, Counterparty>>,
    Counterparty:
        HasPacketDstChannelId<Chain> + HasPacketDstPortId<Chain> + HasPacketSequence<Chain>,
{
    async fn build_timeout_unordered_packet_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        packet: &Counterparty::OutgoingPacket,
    ) -> Result<TimeoutUnorderedPacketPayload<Chain>, Chain::Error> {
        let (receipt, proof_unreceived) = chain
            .query_packet_receipt(
                &Counterparty::packet_dst_channel_id(packet),
                &Counterparty::packet_dst_port_id(packet),
                &Counterparty::packet_sequence(packet),
                height,
            )
            .await?;

        if receipt.is_some() {
            return Err(Chain::raise_error(InvalidTimeoutReceipt {
                chain,
                height,
                packet,
            }));
        }

        // TODO: check that all commitment proof heights are the same
        let update_height = Chain::commitment_proof_height(&proof_unreceived).clone();

        let payload = TimeoutUnorderedPacketPayload {
            update_height,
            proof_unreceived,
        };

        Ok(payload)
    }
}

pub struct InvalidTimeoutReceipt<'a, Chain, Counterparty>
where
    Chain: HasHeightType,
    Counterparty: HasOutgoingPacketType<Chain>,
{
    pub chain: &'a Chain,
    pub height: &'a Chain::Height,
    pub packet: &'a Counterparty::OutgoingPacket,
}

impl<Chain, Counterparty> Debug for InvalidTimeoutReceipt<'_, Chain, Counterparty>
where
    Chain: HasHeightType,
    Counterparty: HasOutgoingPacketType<Chain>
        + HasPacketDstChannelId<Chain>
        + HasPacketDstPortId<Chain>
        + HasPacketSequence<Chain>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f,
            "cannot construct a timeout packet payload as a packet receipt exists at height {} for packet {}/{}/{}",
            self.height,
            Counterparty::packet_dst_channel_id(self.packet),
            Counterparty::packet_dst_port_id(self.packet),
            Counterparty::packet_sequence(self.packet),
        )
    }
}
