use core::marker::PhantomData;

use alloc::vec::Vec;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::fields::transaction::channel_id::HasIbcTransactionChannelIds;
use crate::traits::fields::transaction::header::HasIbcTransactionHeader;
use crate::traits::fields::transaction::messages::HasIbcTransactionMessages;
use crate::traits::handlers::outgoing::message::IbcMessageHandler;
use crate::traits::handlers::outgoing::packet::CanSendPacket;
use crate::traits::handlers::outgoing::transaction::IbcTransactionHandler;
use crate::traits::types::transaction::HasIbcTransactionType;

pub struct HandleMessagesAndSendPacket<App, InHandler>(pub PhantomData<(App, InHandler)>);

impl<Chain, Counterparty, App, InHandler> IbcTransactionHandler<Chain, Counterparty>
    for HandleMessagesAndSendPacket<App, InHandler>
where
    Chain: HasIbcTransactionType<Counterparty>
        + HasIbcTransactionHeader<Counterparty>
        + HasIbcTransactionChannelIds<Counterparty>
        + HasIbcTransactionMessages<Counterparty, App>
        + CanSendPacket<Counterparty>,
    Counterparty: HasChannelIdType<Chain>,
    InHandler: IbcMessageHandler<Chain, Counterparty, App>,
{
    async fn handle_ibc_transaction(
        chain: &Chain,
        transaction: &Chain::IbcTransaction,
    ) -> Result<Chain::Packet, Chain::Error> {
        let transaction_header = Chain::ibc_transcation_header(transaction);

        let messages = Chain::ibc_transcation_messages(transaction);

        let mut payloads = Vec::new();

        for (message_header, message) in messages {
            let payload =
                InHandler::handle_ibc_message(chain, transaction_header, message_header, message)
                    .await?;

            payloads.push(payload);
        }

        chain.send_packet(transaction_header, payloads).await
    }
}
