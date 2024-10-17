use cgp::core::Async;
use hermes_ibc_components::traits::types::payload::data::ProvidePayloadDataType;
use hermes_ibc_token_transfer_components::types::packet_data::transfer::IbcTransferPayloadData;

use crate::contexts::chain::MockChain;

pub enum MockAnyPayloadData<A: Async, B: Async> {
    IbcTransfer(IbcTransferPayloadData<MockChain<A, B>, MockChain<B, A>>),
}

pub struct UseMockAnyPayloadData;

impl<A: Async, B: Async, Counterparty, App>
    ProvidePayloadDataType<MockChain<A, B>, Counterparty, App> for UseMockAnyPayloadData
{
    type PayloadData = MockAnyPayloadData<A, B>;
}

impl<A: Async, B: Async> Clone for MockAnyPayloadData<A, B> {
    fn clone(&self) -> Self {
        match self {
            Self::IbcTransfer(data) => Self::IbcTransfer(data.clone()),
        }
    }
}
