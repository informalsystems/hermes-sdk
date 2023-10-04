use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::queries::write_ack::CanQueryWriteAck;

use crate::one_for_all::traits::chain::{OfaChainTypes, OfaIbcChain};
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::std_prelude::*;

#[async_trait]
impl<Chain, Counterparty> CanQueryWriteAck<OfaChainWrapper<Counterparty>> for OfaChainWrapper<Chain>
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn query_write_ack_event(
        &self,
        packet: &Self::IncomingPacket,
    ) -> Result<Option<Self::WriteAckEvent>, Self::Error> {
        self.chain.query_write_ack_event(packet).await
    }
}
