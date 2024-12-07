use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::message::HasIbcMessageType;
use crate::traits::types::message_header::HasIbcMessageHeaderType;
use crate::traits::types::transaction::HasIbcTransactionType;

#[cgp_component {
  name: IbcTransactionHeaderGetterComponent,
  provider: IbcTransactionHeaderGetter,
  context: Chain,
}]
pub trait HasIbcTransactionMessages<Counterparty, App>:
    HasIbcTransactionType<Counterparty>
    + HasIbcMessageType<Counterparty, App>
    + HasIbcMessageHeaderType<Counterparty>
    + HasAppIdType<Counterparty>
{
    fn ibc_transcation_messages(
        transaction: &Self::IbcTransaction,
    ) -> &[(Self::IbcMessageHeader, Self::IbcMessage)];
}
