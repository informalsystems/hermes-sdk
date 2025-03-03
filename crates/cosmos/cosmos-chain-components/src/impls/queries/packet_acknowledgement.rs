use core::fmt::Display;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::extract_data::CanExtractFromEvent;
use hermes_relayer_components::chain::traits::packet::fields::{
    HasPacketDstChannelId, HasPacketDstPortId, HasPacketSequence,
};
use hermes_relayer_components::chain::traits::queries::block_events::CanQueryBlockEvents;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::{
    PacketAcknowledgementQuerier, PacketAcknowledgementQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::chain::traits::types::packets::ack::{
    HasAckCommitmentHashType, HasAcknowledgementType,
};
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::core::channel::types::packet::Packet;
use ibc::cosmos_host::IBC_QUERY_PATH;

use crate::traits::abci_query::CanQueryAbci;
use crate::types::events::write_acknowledgment::WriteAckEvent;

pub struct QueryPacketAcknowledgementFromAbci;

#[cgp_provider(PacketAcknowledgementQuerierComponent)]
impl<Chain, Counterparty> PacketAcknowledgementQuerier<Chain, Counterparty>
    for QueryPacketAcknowledgementFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId: PartialEq, PortId: PartialEq>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>
        + HasAckCommitmentHashType<AckCommitmentHash = Vec<u8>>
        + CanQueryBlockEvents
        + HasCommitmentProofType
        + CanQueryAbci
        + HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAckEvent>
        + CanExtractFromEvent<Chain::WriteAckEvent>
        + CanRaiseAsyncError<String>,
    Counterparty: HasIbcChainTypes<Chain, Sequence: PartialEq>
        + HasPacketDstChannelId<Chain>
        + HasPacketDstPortId<Chain>
        + HasPacketSequence<Chain>
        + HasOutgoingPacketType<Chain, OutgoingPacket = Packet>,
    Chain::ChannelId: Display,
{
    async fn query_packet_acknowledgement(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequence: &Counterparty::Sequence,
        height: &Chain::Height,
    ) -> Result<Chain::Acknowledgement, Chain::Error> {
        let events = chain.query_block_events(height).await?;

        let ack_event = events
            .iter()
            .filter_map(|event| chain.try_extract_from_event(PhantomData, event))
            .find(|ack_event| {
                &Counterparty::packet_sequence(&ack_event.packet) == sequence
                    && &Counterparty::packet_dst_channel_id(&ack_event.packet) == channel_id
                    && &Counterparty::packet_dst_port_id(&ack_event.packet) == port_id
            })
            .ok_or_else(|| Chain::raise_error(format!("ack event for packet with sequence `{sequence}`, channel id `{channel_id}` and port id `{port_id}` not found")))?
            .acknowledgment;

        Ok(ack_event)
    }

    async fn query_packet_acknowledgement_with_proof(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequence: &Counterparty::Sequence,
        height: &Chain::Height,
    ) -> Result<(Chain::AckCommitmentHash, Chain::CommitmentProof), Chain::Error> {
        let ack_path = format!("acks/ports/{port_id}/channels/{channel_id}/sequences/{sequence}");

        let (ack, proof) = chain
            .query_abci_with_proofs(IBC_QUERY_PATH, ack_path.as_bytes(), height)
            .await?;

        let ack = ack.ok_or_else(|| Chain::raise_error(format!("ack not found at: {ack_path}")))?;

        Ok((ack, proof))
    }
}
