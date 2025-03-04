use cgp::prelude::*;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::{
    PacketAcknowledgementQuerier, PacketAcknowledgementQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAckCommitmentHashType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::cosmos_host::IBC_QUERY_PATH;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryPacketAcknowledgementFromAbci;

#[cgp_provider(PacketAcknowledgementQuerierComponent)]
impl<Chain, Counterparty> PacketAcknowledgementQuerier<Chain, Counterparty>
    for QueryPacketAcknowledgementFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId: PartialEq, PortId: PartialEq>
        + HasAckCommitmentHashType<AckCommitmentHash = Vec<u8>>
        + HasCommitmentProofType
        + CanQueryAbci
        + CanRaiseAsyncError<String>,
    Counterparty: HasIbcChainTypes<Chain, Sequence: PartialEq>,
{
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
