use hermes_prelude::*;

use crate::traits::{
    CanQueryChainHeight, CanQueryPacketCommitment, PacketIsClearedQuerier,
    PacketIsClearedQuerierComponent,
};

#[cgp_new_provider(PacketIsClearedQuerierComponent)]
impl<Chain, Counterparty> PacketIsClearedQuerier<Chain, Counterparty>
    for QueryClearedPacketWithEmptyCommitment
where
    Chain: CanQueryChainHeight + CanQueryPacketCommitment<Counterparty>,
{
    async fn query_packet_is_cleared(
        chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        sequence: &Chain::Sequence,
    ) -> Result<bool, Chain::Error> {
        let height = chain.query_chain_height().await?;

        let (commitment, _proofs) = chain
            .query_packet_commitment(channel_id, port_id, sequence, &height)
            .await?;

        Ok(commitment.is_none())
    }
}
