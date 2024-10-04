use cgp::core::Async;
use hermes_ibc_token_transfer_components::types::packet_data::transfer::IbcTransferPacketData;

use crate::contexts::chain::MockChain;
use crate::types::tagged::Tagged;

pub enum MockAnyPacketData<A: Async, B: Async> {
    IbcTransfer(IbcTransferPacketData<Tagged<A, B, MockChain>, Tagged<B, A, MockChain>>),
}
