use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::components::counterparty_chain_id_querier::CounterpartyChainIdQuerier;

use crate::one_for_all::traits::chain::{OfaChainTypes, OfaIbcChain};
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::one_for_all::types::component::OfaComponents;
use crate::std_prelude::*;

#[async_trait]
impl<Chain, Counterparty>
    CounterpartyChainIdQuerier<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn query_chain_id_from_channel_id(
        chain: &OfaChainWrapper<Chain>,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
    ) -> Result<Counterparty::ChainId, Chain::Error> {
        chain
            .chain
            .query_chain_id_from_channel_id(channel_id, port_id)
            .await
    }
}
