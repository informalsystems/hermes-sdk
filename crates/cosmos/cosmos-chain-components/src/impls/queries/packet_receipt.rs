use core::fmt::Display;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::queries::packet_receipt::{
    PacketReceiptQuerier, PacketReceiptQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasPacketReceiptType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::cosmos_host::IBC_QUERY_PATH;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryPacketReceiptFromAbci;

#[cgp_provider(PacketReceiptQuerierComponent)]
impl<Chain, Counterparty> PacketReceiptQuerier<Chain, Counterparty> for QueryPacketReceiptFromAbci
where
    Chain: HasIbcChainTypes<Counterparty>
        + HasPacketReceiptType<Counterparty, PacketReceipt = Vec<u8>>
        + HasCommitmentProofType
        + CanQueryAbci,
    Counterparty: HasIbcChainTypes<Chain>,
    Chain::ChannelId: Display,
{
    async fn query_packet_receipt(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequence: &Counterparty::Sequence,
        height: &Chain::Height,
    ) -> Result<(Chain::PacketReceipt, Chain::CommitmentProof), Chain::Error> {
        let receipt_path =
            format!("receipts/ports/{port_id}/channels/{channel_id}/sequences/{sequence}");

        let (receipt, proof) = chain
            .query_abci_with_proofs(IBC_QUERY_PATH, receipt_path.as_bytes(), height)
            .await?;

        // TODO: Use a more precise `PacketReceipt` type, i.e. `bool`

        Ok((receipt, proof))
    }
}
