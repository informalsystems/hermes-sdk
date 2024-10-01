use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::message::HasIbcMessageType;
use crate::traits::types::transaction::HasIbcTransactionType;

#[derive_component(IbcTransactionHeaderGetterComponent, IbcTransactionHeaderGetter<Chain>)]
pub trait HasIbcTransactionMessages<Counterparty, App>:
    HasIbcTransactionType<Counterparty>
    + HasIbcMessageType<Counterparty, App>
    + HasAppIdType<Counterparty>
{
    fn ibc_transcation_messages(
        transaction: &Self::IbcTransaction,
    ) -> &[(Self::AppId, Self::IbcMessage)];
}
