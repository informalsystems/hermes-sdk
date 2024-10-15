use cgp::core::component::UseContext;
use cgp::prelude::*;

use crate::traits::types::message::HasIbcMessageType;
use crate::traits::types::message_header::HasIbcMessageHeaderType;
use crate::traits::types::payload::payload::HasPayloadType;
use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

#[derive_component(IbcMessageHandlerComponent, IbcMessageHandler<Chain>)]
#[async_trait]
pub trait CanHandleIbcMessage<Counterparty, App>:
    HasErrorType
    + HasIbcTransactionHeaderType<Counterparty>
    + HasIbcMessageHeaderType<Counterparty>
    + HasIbcMessageType<Counterparty, App>
    + HasPayloadType<Counterparty>
{
    async fn handle_ibc_message(
        &self,
        transaction_header: &Self::IbcTransactionHeader,
        message_header: &Self::IbcMessageHeader,
        message: &Self::IbcMessage,
    ) -> Result<Self::Payload, Self::Error>;
}

impl<Chain, Counterparty, App> IbcMessageHandler<Chain, Counterparty, App> for UseContext
where
    Chain: CanHandleIbcMessage<Counterparty, App>,
{
    async fn handle_ibc_message(
        chain: &Chain,
        transaction_header: &Chain::IbcTransactionHeader,
        message_header: &Chain::IbcMessageHeader,
        message: &Chain::IbcMessage,
    ) -> Result<Chain::Payload, Chain::Error> {
        chain
            .handle_ibc_message(transaction_header, message_header, message)
            .await
    }
}
