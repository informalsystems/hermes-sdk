use cgp::prelude::*;

use crate::traits::types::message::HasIbcMessageType;
use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

#[derive_component(IbcMessageHandlerComponent, IbcMessageHandler<Chain>)]
#[async_trait]
pub trait CanHandleIbcMessage<Counterparty, App>:
    HasErrorType
    + HasIbcTransactionHeaderType<Counterparty>
    + HasIbcMessageType<Counterparty, App>
    + HasPacketDataType<Counterparty, App>
    + HasPacketEntryHeaderType<Counterparty>
{
    async fn handle_ibc_message(
        &self,
        tx_header: &Self::IbcTransactionHeader,
        message: &Self::IbcMessage,
    ) -> Result<(Self::PacketEntryHeader, Self::PacketData), Self::Error>;
}
