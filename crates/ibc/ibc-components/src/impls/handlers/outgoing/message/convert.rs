use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::core::Async;
use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasEncoding;

use crate::traits::handlers::outgoing::message::IbcMessageHandler;
use crate::traits::types::message::HasIbcMessageType;
use crate::traits::types::message_header::HasIbcMessageHeaderType;
use crate::traits::types::payload::header::HasPayloadHeaderType;
use crate::traits::types::payload::payload::HasPayloadType;
use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

pub struct ConvertAndHandleIbcMessage<InApp, InHandler>(pub PhantomData<(InApp, InHandler)>);

impl<Chain, Counterparty, App, InApp, InHandler, AnyMessage, Message>
    IbcMessageHandler<Chain, Counterparty, App> for ConvertAndHandleIbcMessage<InApp, InHandler>
where
    Chain: HasErrorType
        + HasIbcTransactionHeaderType<Counterparty>
        + HasIbcMessageHeaderType<Counterparty>
        + HasIbcMessageType<Counterparty, App, IbcMessage = AnyMessage>
        + HasIbcMessageType<Counterparty, InApp, IbcMessage = Message>
        + HasPayloadType<Counterparty>
        + HasPayloadHeaderType<Counterparty>
        + HasEncoding<App>
        + CanRaiseError<ErrorOf<Chain::Encoding>>,
    InHandler: IbcMessageHandler<Chain, Counterparty, InApp>,
    Chain::Encoding: CanConvert<AnyMessage, Message>,
    Message: Async,
    AnyMessage: Async,
{
    async fn handle_ibc_message(
        chain: &Chain,
        transaction_header: &Chain::IbcTransactionHeader,
        message_header: &Chain::IbcMessageHeader,
        any_message: &AnyMessage,
    ) -> Result<Chain::Payload, Chain::Error> {
        let encoding = chain.encoding();

        let message = encoding.convert(any_message).map_err(Chain::raise_error)?;

        InHandler::handle_ibc_message(chain, transaction_header, message_header, &message).await
    }
}
