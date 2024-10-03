use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;

use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

#[derive_component(IbcTransactionCalleGetterComponent, IbcTransactionCalleGetter<Chain>)]
pub trait HasIbcTransactionCaller<Counterparty>:
    HasIbcTransactionHeaderType<Counterparty> + HasAddressType
{
    fn ibc_transaction_caller(transaction_header: &Self::IbcTransactionHeader) -> &Self::Address;
}
