use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::handlers::incoming::payload::{
    IncomingPayloadHandler, IncomingPayloadHandlerComponent,
};
use hermes_ibc_components::traits::types::packet::header::HasPacketHeaderType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;
use hermes_ibc_components::traits::types::payload::header::HasPayloadHeaderType;

use crate::types::packet_data::mint::IbcTransferMintPayloadData;
use crate::types::packet_data::transfer::IbcTransferPayloadData;
use crate::types::packet_data::unescrow::IbcTransferUnescrowPayloadData;
use crate::types::tags::{IbcTransferMintApp, IbcTransferUnescrowApp};

pub struct DispatchMintOrUnescrow<MintHandler, UnescrowHandler>(
    pub PhantomData<(MintHandler, UnescrowHandler)>,
);

#[cgp_provider(IncomingPayloadHandlerComponent)]
impl<Chain, Counterparty, App, MintHandler, UnescrowHandler>
    IncomingPayloadHandler<Chain, Counterparty, App>
    for DispatchMintOrUnescrow<MintHandler, UnescrowHandler>
where
    Chain: HasAsyncErrorType + HasAmountType + HasAddressType,
    Counterparty: HasAmountType
        + HasPacketHeaderType<Chain>
        + HasPayloadHeaderType<Chain>
        + HasPayloadDataType<Chain, App, PayloadData = IbcTransferPayloadData<Counterparty, Chain>>
        + HasPayloadDataType<
            Chain,
            IbcTransferMintApp,
            PayloadData = IbcTransferMintPayloadData<Counterparty, Chain>,
        > + HasPayloadDataType<
            Chain,
            IbcTransferUnescrowApp,
            PayloadData = IbcTransferUnescrowPayloadData<Counterparty, Chain>,
        >,
    MintHandler: IncomingPayloadHandler<Chain, Counterparty, IbcTransferMintApp>,
    UnescrowHandler: IncomingPayloadHandler<Chain, Counterparty, IbcTransferUnescrowApp>,
{
    async fn handle_incoming_payload(
        chain: &mut Chain,
        packet_header: &Counterparty::PacketHeader,
        payload_header: &Counterparty::PayloadHeader,
        payload_data: &IbcTransferPayloadData<Counterparty, Chain>,
    ) -> Result<(), Chain::Error> {
        match payload_data {
            IbcTransferPayloadData::Mint(payload) => {
                MintHandler::handle_incoming_payload(chain, packet_header, payload_header, payload)
                    .await
            }
            IbcTransferPayloadData::Unescrow(payload) => {
                UnescrowHandler::handle_incoming_payload(
                    chain,
                    packet_header,
                    payload_header,
                    payload,
                )
                .await
            }
        }
    }
}
