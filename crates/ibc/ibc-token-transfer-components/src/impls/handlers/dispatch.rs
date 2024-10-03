use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::handlers::incoming::payload::IncomingPayloadHandler;
use hermes_ibc_components::traits::types::packet::header::HasPacketHeaderType;
use hermes_ibc_components::traits::types::payload::ack::HasPayloadAckType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;
use hermes_ibc_components::traits::types::payload::header::HasPayloadHeaderType;

use crate::traits::parse::{
    CanParseIncomingTransferData, HasIncomingTransferApps, IncomingTransferData,
};

pub struct DispatchMintOrUnescrow<MintHandler, UnescrowHandler>(
    pub PhantomData<(MintHandler, UnescrowHandler)>,
);

impl<
        Chain,
        Counterparty,
        App,
        MintApp,
        UnescrowApp,
        MintHandler,
        UnescrowHandler,
        TransferPayload,
        MintPayload,
        UnescrowPayload,
        Ack,
    > IncomingPayloadHandler<Chain, Counterparty, App>
    for DispatchMintOrUnescrow<MintHandler, UnescrowHandler>
where
    Chain: HasErrorType
        + HasAmountType
        + HasIncomingTransferApps<MintApp = MintApp, UnescrowApp = UnescrowApp>
        + HasPayloadAckType<Counterparty, App, PayloadAck = Ack>
        + HasPayloadAckType<Counterparty, MintApp, PayloadAck = Ack>
        + HasPayloadAckType<Counterparty, UnescrowApp, PayloadAck = Ack>
        + CanParseIncomingTransferData<Counterparty, App>,
    Counterparty: HasAmountType
        + HasPacketHeaderType<Chain>
        + HasPayloadHeaderType<Chain>
        + HasPayloadDataType<Chain, App, PayloadData = TransferPayload>
        + HasPayloadDataType<Chain, MintApp, PayloadData = MintPayload>
        + HasPayloadDataType<Chain, UnescrowApp, PayloadData = UnescrowPayload>,
    MintHandler: IncomingPayloadHandler<Chain, Counterparty, MintApp>,
    UnescrowHandler: IncomingPayloadHandler<Chain, Counterparty, UnescrowApp>,
    TransferPayload: Async,
    MintPayload: Async,
    UnescrowPayload: Async,
{
    async fn handle_incoming_payload(
        chain: &Chain,
        packet_header: &Counterparty::PacketHeader,
        payload_header: &Counterparty::PayloadHeader,
        payload_data: &TransferPayload,
    ) -> Result<Ack, Chain::Error> {
        let payload = chain.parse_incoming_transfer_data(payload_data)?;

        match payload {
            IncomingTransferData::Mint(payload) => {
                MintHandler::handle_incoming_payload(chain, packet_header, payload_header, &payload)
                    .await
            }
            IncomingTransferData::Unescrow(payload) => {
                UnescrowHandler::handle_incoming_payload(
                    chain,
                    packet_header,
                    payload_header,
                    &payload,
                )
                .await
            }
        }
    }
}
