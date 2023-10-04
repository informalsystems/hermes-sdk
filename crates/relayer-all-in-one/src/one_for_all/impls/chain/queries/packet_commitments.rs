use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::components::packet_commitments_querier::PacketCommitmentsQuerier;

use crate::one_for_all::traits::chain::OfaIbcChain;
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::one_for_all::types::component::OfaComponents;
use crate::std_prelude::*;

#[async_trait]
impl<Chain, Counterparty>
    PacketCommitmentsQuerier<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaIbcChain<Chain>,
{
    /// Query the sequences of the packets that the chain has committed to be
    /// sent to the counterparty chain, of which the full packet relaying is not
    /// yet completed. Once the chain receives the ack from the counterparty
    /// chain, a given sequence should be removed from the packet commitment list.
    async fn query_packet_commitments(
        chain: &OfaChainWrapper<Chain>,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
    ) -> Result<(Vec<Chain::Sequence>, Chain::Height), Chain::Error> {
        let commitments = chain
            .chain
            .query_packet_commitments(channel_id, port_id)
            .await?;

        Ok(commitments)
    }
}
