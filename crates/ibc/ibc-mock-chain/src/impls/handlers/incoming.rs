use alloc::string::String;

use hermes_ibc_components::traits::handlers::incoming::payload::{
    CanHandleIncomingPayload, IncomingPayloadHandler, IncomingPayloadHandlerComponent,
};
use hermes_ibc_components::types::packet_header::IbcPacketHeader;
use hermes_ibc_components::types::payload_header::IbcPayloadHeader;
use hermes_ibc_components::types::tags::apps::any::AnyApp;
use hermes_ibc_token_transfer_components::types::tags::IbcTransferApp;
use hermes_prelude::*;

use crate::contexts::chain::MockChain;
use crate::types::packet_data::MockAnyPayloadData;

pub struct HandleMockAnyPayloadData;

#[cgp_provider(IncomingPayloadHandlerComponent)]
impl<Chain: Async, Counterparty: Async>
    IncomingPayloadHandler<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>, AnyApp>
    for HandleMockAnyPayloadData
{
    async fn handle_incoming_payload(
        chain: &mut MockChain<Chain, Counterparty>,
        packet_header: &IbcPacketHeader<
            MockChain<Counterparty, Chain>,
            MockChain<Chain, Counterparty>,
        >,
        payload_header: &IbcPayloadHeader<
            MockChain<Counterparty, Chain>,
            MockChain<Chain, Counterparty>,
        >,
        payload_data: &MockAnyPayloadData<Counterparty, Chain>,
    ) -> Result<(), String> {
        match payload_data {
            MockAnyPayloadData::IbcTransfer(payload_data) => {
                <MockChain<Chain, Counterparty> as CanHandleIncomingPayload<
                    MockChain<Counterparty, Chain>,
                    IbcTransferApp,
                >>::handle_incoming_payload(
                    chain, packet_header, payload_header, payload_data
                )
                .await
            }
        }
    }
}
