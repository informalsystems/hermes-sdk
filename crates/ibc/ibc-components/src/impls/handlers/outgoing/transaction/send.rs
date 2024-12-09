use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::builders::payload::CanBuildPayload;
use crate::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use crate::traits::fields::transaction::header::HasIbcTransactionHeader;
use crate::traits::fields::transaction::messages::HasIbcTransactionMessages;
use crate::traits::handlers::outgoing::message::IbcMessageHandler;
use crate::traits::handlers::outgoing::packet::CanSendPacket;
use crate::traits::handlers::outgoing::transaction::IbcTransactionHandler;
use crate::traits::types::transaction::HasIbcTransactionType;

pub struct HandleMessagesAndSendPacket<App, InHandler>(pub PhantomData<(App, InHandler)>);

#[async_trait]
impl<Chain, Counterparty, App, InHandler> IbcTransactionHandler<Chain, Counterparty>
    for HandleMessagesAndSendPacket<App, InHandler>
where
    Chain: HasIbcTransactionType<Counterparty>
        + HasIbcTransactionHeader<Counterparty>
        + HasPacketChannelIds<Counterparty>
        + HasIbcTransactionMessages<Counterparty, App>
        + CanBuildPayload<Counterparty, App>
        + CanSendPacket<Counterparty>,
    Counterparty: HasChannelIdType<Chain>,
    InHandler: IbcMessageHandler<Chain, Counterparty, App>,
{
    async fn handle_ibc_transaction(
        chain: &mut Chain,
        transaction: &Chain::IbcTransaction,
    ) -> Result<Chain::Packet, Chain::Error> {
        let packet_header = Chain::ibc_transcation_header(transaction);

        let messages = Chain::ibc_transcation_messages(transaction);

        let mut payloads = Vec::new();

        for (message_header, message) in messages {
            let (payload_header, payload_data) =
                InHandler::handle_ibc_message(chain, packet_header, message_header, message)
                    .await?;

            let payload = Chain::build_payload(payload_header, payload_data)?;

            payloads.push(payload);
        }

        chain.send_packet(packet_header, payloads).await
    }
}
