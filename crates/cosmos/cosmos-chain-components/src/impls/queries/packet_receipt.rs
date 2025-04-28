use hermes_core::relayer_components::chain::traits::{
    HasCommitmentProofType, HasIbcChainTypes, HasPacketReceiptType, PacketReceiptQuerier,
    PacketReceiptQuerierComponent,
};
use hermes_prelude::*;
use ibc::cosmos_host::IBC_QUERY_PATH;

use crate::traits::CanQueryAbci;

pub struct QueryPacketReceiptFromAbci;

#[cgp_provider(PacketReceiptQuerierComponent)]
impl<Chain, Counterparty> PacketReceiptQuerier<Chain, Counterparty> for QueryPacketReceiptFromAbci
where
    Chain: HasIbcChainTypes<Counterparty>
        + HasPacketReceiptType<Counterparty, PacketReceipt = Vec<u8>>
        + HasCommitmentProofType
        + CanQueryAbci
        + CanRaiseAsyncError<String>,
    Counterparty: HasIbcChainTypes<Chain>,
{
    async fn query_packet_receipt(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequence: &Counterparty::Sequence,
        height: &Chain::Height,
    ) -> Result<(Option<Chain::PacketReceipt>, Chain::CommitmentProof), Chain::Error> {
        let receipt_path =
            format!("receipts/ports/{port_id}/channels/{channel_id}/sequences/{sequence}");

        let (receipt, proof) = chain
            .query_abci_with_proofs(IBC_QUERY_PATH, receipt_path.as_bytes(), height)
            .await?;

        // TODO: Use a more precise `PacketReceipt` type, i.e. `bool`

        Ok((receipt, proof))
    }
}
