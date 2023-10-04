use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::components::unreceived_packet_sequences_querier::UnreceivedPacketSequencesQuerier;

use crate::one_for_all::traits::chain::{OfaChainTypes, OfaIbcChain};
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::one_for_all::types::component::OfaComponents;
use crate::std_prelude::*;

#[async_trait]
impl<Chain, Counterparty>
    UnreceivedPacketSequencesQuerier<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty> + OfaChainTypes,
    Counterparty: OfaIbcChain<Chain>,
{
    /// Given a list of counterparty commitment sequences,
    /// return a filtered list of sequences which the chain
    /// has not received the packet from the counterparty chain.
    async fn query_unreceived_packet_sequences(
        chain: &OfaChainWrapper<Chain>,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequences: &[Counterparty::Sequence],
    ) -> Result<Vec<Counterparty::Sequence>, Chain::Error> {
        let unreceived_packet_sequences = chain
            .chain
            .query_unreceived_packet_sequences(channel_id, port_id, sequences)
            .await?;

        Ok(unreceived_packet_sequences)
    }
}
