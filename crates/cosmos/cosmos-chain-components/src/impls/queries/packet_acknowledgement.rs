use core::fmt::Display;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::{
    PacketAcknowledgementQuerier, PacketAcknowledgementQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAcknowledgementType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::cosmos_host::IBC_QUERY_PATH;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryPacketAcknowledgementFromAbci;

#[cgp_provider(PacketAcknowledgementQuerierComponent)]
impl<Chain, Counterparty> PacketAcknowledgementQuerier<Chain, Counterparty>
    for QueryPacketAcknowledgementFromAbci
where
    Chain: HasIbcChainTypes<Counterparty>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>
        + HasCommitmentProofType
        + CanQueryAbci
        + HasAsyncErrorType,
    Counterparty: HasIbcChainTypes<Chain>,
    Chain::ChannelId: Display,
{
    async fn query_packet_acknowledgement(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequence: &Counterparty::Sequence,
        height: &Chain::Height,
    ) -> Result<(Chain::Acknowledgement, Chain::CommitmentProof), Chain::Error> {
        let ack_path = format!("acks/ports/{port_id}/channels/{channel_id}/sequences/{sequence}");

        let (ack, proof) = chain
            .query_abci_with_proofs(IBC_QUERY_PATH, ack_path.as_bytes(), height)
            .await?;

        Ok((ack, proof))
    }
}
