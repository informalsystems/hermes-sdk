use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::transaction::HasIbcTransactionType;

#[derive_component(IbcTransactionHandlerComponent, IbcTransactionHandler<Chain>)]
#[async_trait]
pub trait CanHandleIbcTransaction<Counterparty>:
    HasErrorType + HasIbcTransactionType<Counterparty> + HasPacketType<Counterparty>
where
    Counterparty: HasClientIdType<Self>,
{
    async fn handle_ibc_transaction(
        &self,
        transaction: &Self::IbcTransaction,
    ) -> Result<Self::Packet, Self::Error>;
}
