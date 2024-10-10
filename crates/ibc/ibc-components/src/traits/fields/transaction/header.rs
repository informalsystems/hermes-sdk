use cgp::prelude::*;

use crate::traits::types::transaction::HasIbcTransactionType;
use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

#[derive_component(IbcTransactionHeaderGetterComponent, IbcTransactionHeaderGetter<Chain>)]
pub trait HasIbcTransactionHeader<Counterparty>:
    HasIbcTransactionType<Counterparty> + HasIbcTransactionHeaderType<Counterparty>
{
    fn ibc_transcation_header(transaction: &Self::IbcTransaction) -> &Self::IbcTransactionHeader;
}
