use cgp::core::Async;
use hermes_ibc_components::traits::types::payload::data::ProvidePayloadDataType;
use hermes_ibc_token_transfer_components::types::packet_data::transfer::IbcTransferPayloadData;

use crate::contexts::chain::MockChain;
use crate::types::tagged::Tagged;

pub enum MockAnyPayloadData<A: Async, B: Async> {
    IbcTransfer(IbcTransferPayloadData<Tagged<A, B, MockChain>, Tagged<B, A, MockChain>>),
}

pub struct ProvideMockAnyPayloadData;

impl<A: Async, B: Async, App>
    ProvidePayloadDataType<Tagged<A, B, MockChain>, Tagged<B, A, MockChain>, App>
    for ProvideMockAnyPayloadData
{
    type PayloadData = MockAnyPayloadData<A, B>;
}
