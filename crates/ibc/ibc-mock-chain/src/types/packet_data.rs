use hermes_ibc_components::traits::types::payload::data::{
    PayloadDataTypeComponent, ProvidePayloadDataType,
};
use hermes_ibc_token_transfer_components::types::packet_data::transfer::IbcTransferPayloadData;
use hermes_prelude::*;

use crate::contexts::chain::MockChain;

pub enum MockAnyPayloadData<A: Async, B: Async> {
    IbcTransfer(IbcTransferPayloadData<MockChain<A, B>, MockChain<B, A>>),
}

pub struct UseMockAnyPayloadData;

#[cgp_provider(PayloadDataTypeComponent)]
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

impl<A: Async, B: Async> PartialEq for MockAnyPayloadData<A, B> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::IbcTransfer(data), Self::IbcTransfer(other)) => data == other,
        }
    }
}

impl<A: Async, B: Async> Eq for MockAnyPayloadData<A, B> {}
