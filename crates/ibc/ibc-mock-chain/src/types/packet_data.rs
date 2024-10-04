use hermes_ibc_token_transfer_components::types::packet_data::transfer::IbcTransferPacketData;

use crate::contexts::chain::MockChain;

pub enum MockAnyPacketData {
    IbcTransfer(IbcTransferPacketData<MockChain, MockChain>),
}
