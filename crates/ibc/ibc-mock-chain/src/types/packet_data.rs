use cgp::core::Async;
use hermes_ibc_components::traits::types::payload::data::ProvidePayloadDataType;
use hermes_ibc_token_transfer_components::types::packet_data::transfer::IbcTransferPayloadData;

use crate::contexts::chain::MockChain;

pub enum MockAnyPayloadData<A: Async, B: Async> {
    IbcTransfer(IbcTransferPayloadData<MockChain<A, B>, MockChain<B, A>>),
}

pub struct UseMockAnyPayloadData;

impl<A: Async, B: Async, App> ProvidePayloadDataType<MockChain<A, B>, MockChain<B, A>, App>
    for UseMockAnyPayloadData
{
    type PayloadData = MockAnyPayloadData<A, B>;
}
