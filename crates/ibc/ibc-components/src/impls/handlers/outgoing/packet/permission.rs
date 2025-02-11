use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::*;

use crate::traits::fields::caller::HasCaller;
use crate::traits::fields::payload::header::HasPayloadHeader;
use crate::traits::handlers::outgoing::packet::{PacketSender, PacketSenderComponent};
use crate::traits::handlers::outgoing::permission::CanCheckSendPayloadPermission;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::packet::HasPacketType;

pub struct CheckSendPayloadPermission<InHandler>(pub PhantomData<InHandler>);

#[cgp_provider(PacketSenderComponent)]
#[async_trait]
impl<Chain, Counterparty, InHandler> PacketSender<Chain, Counterparty>
    for CheckSendPayloadPermission<InHandler>
where
    Chain: HasPacketHeaderType<Counterparty>
        + HasPayloadHeader<Counterparty>
        + HasPacketType<Counterparty>
        + HasCaller
        + CanCheckSendPayloadPermission<Counterparty>,
    InHandler: PacketSender<Chain, Counterparty>,
{
    async fn send_packet(
        chain: &mut Chain,
        packet_header: &Chain::PacketHeader,
        payloads: Vec<Chain::Payload>,
    ) -> Result<Chain::Packet, Chain::Error> {
        let sender = chain.caller();

        for payload in payloads.iter() {
            let payload_header = Chain::payload_header(payload);

            chain
                .check_send_payload_permission(&sender, payload_header)
                .await?;
        }

        InHandler::send_packet(chain, packet_header, payloads).await
    }
}
