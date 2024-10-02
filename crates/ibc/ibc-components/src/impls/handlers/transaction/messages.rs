use core::marker::PhantomData;

use alloc::vec::Vec;
use cgp::prelude::HasErrorType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::builders::packet::CanBuildPacket;
use crate::traits::fields::transaction::channel::HasIbcTransactionChannels;
use crate::traits::fields::transaction::header::HasIbcTransactionHeader;
use crate::traits::fields::transaction::messages::HasIbcTransactionMessages;
use crate::traits::handlers::message::CanHandleIbcMessage;
use crate::traits::handlers::transaction::IbcTransactionHandler;
use crate::traits::nonce::CanAllocatePacketNonce;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::transaction::HasIbcTransactionType;

pub struct HandleIbcTransactionMessages<App>(pub PhantomData<App>);

impl<Chain, Counterparty, App> IbcTransactionHandler<Chain, Counterparty>
    for HandleIbcTransactionMessages<App>
where
    Chain: HasErrorType
        + HasIbcTransactionType<Counterparty>
        + HasPacketType<Counterparty>
        + HasIbcTransactionHeader<Counterparty>
        + HasIbcTransactionChannels<Counterparty>
        + HasIbcTransactionMessages<Counterparty, App>
        + CanHandleIbcMessage<Counterparty, App>
        + CanBuildPacket<Counterparty, App>
        + CanAllocatePacketNonce<Counterparty>
        + HasChannelIdType<Counterparty>,
    Counterparty: HasChannelIdType<Chain>,
{
    async fn handle_ibc_transaction(
        chain: &Chain,
        transaction: &Chain::IbcTransaction,
    ) -> Result<Chain::Packet, Chain::Error> {
        let transaction_header = Chain::ibc_transcation_header(transaction);
        let src_channel_id = Chain::transaction_src_channel_id(transaction_header);
        let dst_channel_id = Chain::transaction_dst_channel_id(transaction_header);

        let messages = Chain::ibc_transcation_messages(transaction);

        let mut entries = Vec::new();

        for (message_header, message) in messages {
            let entry = chain
                .handle_ibc_message(transaction_header, message_header, message)
                .await?;

            entries.push(entry);
        }

        let nonce = chain
            .allocate_packet_nonce(src_channel_id, dst_channel_id)
            .await?;

        let packet = chain
            .build_packet(transaction_header, nonce, entries)
            .await?;

        Ok(packet)
    }
}
