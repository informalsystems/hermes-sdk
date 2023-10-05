use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::components::send_packets_querier::SendPacketsQuerier;

use crate::one_for_all::traits::chain::{OfaChainTypes, OfaIbcChain};
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::one_for_all::types::component::OfaComponents;
use crate::std_prelude::*;

#[async_trait]
impl<Chain, Counterparty> SendPacketsQuerier<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty> + OfaChainTypes,
    Counterparty: OfaIbcChain<Chain>,
{
    /// Given a list of sequences, a channel and port will query a list of outgoing
    /// packets which have not been relayed.
    async fn query_send_packets_from_sequences(
        chain: &OfaChainWrapper<Chain>,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_port_id: &Counterparty::PortId,
        sequences: &[Chain::Sequence],
        // The height is given to query the packets from a specific height.
        // This height should be the same as the query height from the
        // `CanQueryPacketCommitments` made on the same chain.
        height: &Chain::Height,
    ) -> Result<Vec<Chain::OutgoingPacket>, Chain::Error> {
        let send_packets = chain
            .chain
            .query_send_packets_from_sequences(
                channel_id,
                port_id,
                counterparty_channel_id,
                counterparty_port_id,
                sequences,
                height,
            )
            .await?;
        Ok(send_packets)
    }
}
