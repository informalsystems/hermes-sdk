use cgp_core::prelude::HasErrorType;

use crate::chain::traits::packet::fields::CanReadPacketFields;
use crate::chain::traits::payload_builders::ack_packet::AckPacketPayloadBuilder;
use crate::chain::traits::queries::packet_acknowledgement::CanQueryPacketAcknowledgement;
use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::height::CanIncrementHeight;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::packets::ack::HasAckPacketPayloadType;
use crate::chain::traits::types::packets::ack::HasAcknowledgementType;
use crate::chain::traits::types::proof::HasCommitmentProofType;
use crate::chain::types::payloads::packet::AckPacketPayload;

pub struct BuildPacketPayloads;

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

        let payload = AckPacketPayload {
            ack: ack.clone(),
            update_height,
            proof_ack,
        };

        Ok(payload)
    }
}
