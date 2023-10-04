use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::components::write_ack_querier::WriteAckQuerier;

use crate::one_for_all::traits::chain::{OfaChainTypes, OfaIbcChain};
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::one_for_all::types::component::OfaComponents;
use crate::std_prelude::*;

#[async_trait]
impl<Chain, Counterparty> WriteAckQuerier<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn query_write_ack_event(
        chain: &OfaChainWrapper<Chain>,
        packet: &Chain::IncomingPacket,
    ) -> Result<Option<Chain::WriteAckEvent>, Chain::Error> {
        chain.chain.query_write_ack_event(packet).await
    }
}
