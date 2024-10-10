use cgp::prelude::*;

use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::transaction::HasIbcTransactionType;

#[derive_component(IbcTransactionHandlerComponent, IbcTransactionHandler<Chain>)]
#[async_trait]
pub trait CanHandleIbcTransaction<Counterparty>:
    HasErrorType + HasIbcTransactionType<Counterparty> + HasPacketType<Counterparty>
{
    async fn handle_ibc_transaction(
        &self,
        transaction: &Self::IbcTransaction,
    ) -> Result<Self::Packet, Self::Error>;
}
