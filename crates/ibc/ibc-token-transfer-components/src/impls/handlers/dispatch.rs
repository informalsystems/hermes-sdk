use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::handlers::incoming::payload::IncomingPayloadHandler;
use hermes_ibc_components::traits::types::packet::header::HasPacketHeaderType;
use hermes_ibc_components::traits::types::payload::ack::HasPayloadAckType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;
use hermes_ibc_components::traits::types::payload::header::HasPayloadHeaderType;

use crate::traits::fields::payload_data::amount::{HasIbcTransferAmount, IbcTransferAmount};

pub struct DispatchMintOrUnescrow<MintApp, UnescrowApp, MintHandler, UnescrowHandler>(
    pub PhantomData<(MintApp, UnescrowApp, MintHandler, UnescrowHandler)>,
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
        Ack,
    > IncomingPayloadHandler<Chain, Counterparty, App>
    for DispatchMintOrUnescrow<MintApp, UnescrowApp, MintHandler, UnescrowHandler>
where
    Chain: HasErrorType
        + HasPayloadAckType<Counterparty, App, PayloadAck = Ack>
        + HasPayloadAckType<Counterparty, MintApp, PayloadAck = Ack>
        + HasPayloadAckType<Counterparty, UnescrowApp, PayloadAck = Ack>
        + HasAmountType,
    Counterparty: HasAmountType
        + HasPacketHeaderType<Chain>
        + HasPayloadHeaderType<Chain>
        + HasPayloadDataType<Chain, App, PayloadData = TransferPayload>
        + HasPayloadDataType<Chain, MintApp, PayloadData = (TransferPayload, Counterparty::Amount)>
        + HasPayloadDataType<Chain, UnescrowApp, PayloadData = (TransferPayload, Chain::Amount)>
        + HasIbcTransferAmount<Chain, App>,
    MintHandler: IncomingPayloadHandler<Chain, Counterparty, MintApp>,
    UnescrowHandler: IncomingPayloadHandler<Chain, Counterparty, UnescrowApp>,
    TransferPayload: Async + Clone,
    Chain::Amount: Clone,
    Counterparty::Amount: Clone,
{
    async fn handle_incoming_payload(
        chain: &Chain,
        packet_header: &Counterparty::PacketHeader,
        payload_header: &Counterparty::PayloadHeader,
        payload_data: &TransferPayload,
    ) -> Result<Ack, Chain::Error> {
        let amount = Counterparty::ibc_transfer_amount(payload_data);

        match amount {
            IbcTransferAmount::Mint(amount) => {
                MintHandler::handle_incoming_payload(
                    chain,
                    packet_header,
                    payload_header,
                    &(payload_data.clone(), amount.clone()),
                )
                .await
            }
            IbcTransferAmount::Unescrow(amount) => {
                UnescrowHandler::handle_incoming_payload(
                    chain,
                    packet_header,
                    payload_header,
                    &(payload_data.clone(), amount.clone()),
                )
                .await
            }
        }
    }
}
