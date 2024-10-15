use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::traits::fields::caller::HasCaller;
use crate::traits::fields::payload::header::HasPayloadHeader;
use crate::traits::handlers::outgoing::packet::PacketSender;
use crate::traits::handlers::outgoing::permission::CanCheckSendPayloadPermission;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

pub struct CheckSendPayloadPermission<InHandler>(pub PhantomData<InHandler>);

impl<Chain, Counterparty, InHandler> PacketSender<Chain, Counterparty>
    for CheckSendPayloadPermission<InHandler>
where
    Chain: HasIbcTransactionHeaderType<Counterparty>
        + HasPayloadHeader<Counterparty>
        + HasPacketType<Counterparty>
        + HasCaller
        + CanCheckSendPayloadPermission<Counterparty>,
    InHandler: PacketSender<Chain, Counterparty>,
{
    async fn send_packet(
        chain: &Chain,
        transaction_header: &Chain::IbcTransactionHeader,
        payloads: Vec<Chain::Payload>,
    ) -> Result<Chain::Packet, Chain::Error> {
        let sender = chain.caller();

        for payload in payloads.iter() {
            let payload_header = Chain::payload_header(payload);

            chain
                .check_send_payload_permission(sender, payload_header)
                .await?;
        }

        InHandler::send_packet(chain, transaction_header, payloads).await
    }
}
