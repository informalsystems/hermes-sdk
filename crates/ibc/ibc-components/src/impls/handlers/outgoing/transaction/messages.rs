use core::marker::PhantomData;

use alloc::vec::Vec;
use cgp::prelude::HasErrorType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::builders::packet::CanBuildPacket;
use crate::traits::fields::transaction::channel::HasIbcTransactionChannels;
use crate::traits::fields::transaction::header::HasIbcTransactionHeader;
use crate::traits::fields::transaction::messages::HasIbcTransactionMessages;
use crate::traits::handlers::outgoing::message::IbcMessageHandler;
use crate::traits::handlers::outgoing::transaction::IbcTransactionHandler;
use crate::traits::nonce::CanAllocatePacketNonce;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::transaction::HasIbcTransactionType;

pub struct HandleIbcTransactionMessages<App, InHandler>(pub PhantomData<(App, InHandler)>);

impl<Chain, Counterparty, App, InHandler> IbcTransactionHandler<Chain, Counterparty>
    for HandleIbcTransactionMessages<App, InHandler>
where
    Chain: HasErrorType
        + HasIbcTransactionType<Counterparty>
        + HasPacketType<Counterparty>
        + HasIbcTransactionHeader<Counterparty>
        + HasIbcTransactionChannels<Counterparty>
        + HasIbcTransactionMessages<Counterparty, App>
        + CanBuildPacket<Counterparty, App>
        + CanAllocatePacketNonce<Counterparty>
        + HasChannelIdType<Counterparty>,
    Counterparty: HasChannelIdType<Chain>,
    InHandler: IbcMessageHandler<Chain, Counterparty, App>,
{
    async fn handle_ibc_transaction(
        chain: &Chain,
        transaction: &Chain::IbcTransaction,
    ) -> Result<Chain::Packet, Chain::Error> {
        let transaction_header = Chain::ibc_transcation_header(transaction);
        let src_channel_id = Chain::transaction_src_channel_id(transaction_header);
        let dst_channel_id = Chain::transaction_dst_channel_id(transaction_header);

        let messages = Chain::ibc_transcation_messages(transaction);

        let mut payloads = Vec::new();

        for (message_header, message) in messages {
            let payload =
                InHandler::handle_ibc_message(chain, transaction_header, message_header, message)
                    .await?;

            payloads.push(payload);
        }

        let nonce = chain
            .allocate_packet_nonce(src_channel_id, dst_channel_id)
            .await?;

        let packet = chain
            .build_packet(transaction_header, nonce, payloads)
            .await?;

        Ok(packet)
    }
}
