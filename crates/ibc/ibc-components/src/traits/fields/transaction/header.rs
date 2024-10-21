use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::transaction::HasIbcTransactionType;

#[derive_component(IbcTransactionHeaderGetterComponent, IbcTransactionHeaderGetter<Chain>)]
pub trait HasIbcTransactionHeader<Counterparty>:
    HasIbcTransactionType<Counterparty> + HasPacketHeaderType<Counterparty>
{
    fn ibc_transcation_header(transaction: &Self::IbcTransaction) -> &Self::PacketHeader;
}
