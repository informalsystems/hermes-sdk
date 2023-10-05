use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::components::received_packet_querier::ReceivedPacketQuerier;

use crate::one_for_all::traits::chain::{OfaChainTypes, OfaIbcChain};
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::one_for_all::types::component::OfaComponents;
use crate::std_prelude::*;

#[async_trait]
impl<Chain, Counterparty>
    ReceivedPacketQuerier<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>> for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn query_is_packet_received(
        chain: &OfaChainWrapper<Chain>,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        sequence: &Counterparty::Sequence,
    ) -> Result<bool, Chain::Error> {
        let is_received = chain
            .chain
            .query_is_packet_received(port_id, channel_id, sequence)
            .await?;

        Ok(is_received)
    }
}
