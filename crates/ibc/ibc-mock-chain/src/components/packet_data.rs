use cgp::prelude::*;
use hermes_ibc_components::types::any_app::AnyApp;
use hermes_ibc_token_transfer_components::types::packet_data::mint::IbcTransferMintPacketData;
use hermes_ibc_token_transfer_components::types::packet_data::transfer::IbcTransferPacketData;
use hermes_ibc_token_transfer_components::types::packet_data::unescrow::IbcTransferUnescrowPacketData;
use hermes_ibc_token_transfer_components::types::tags::{
    IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp,
};

use crate::contexts::chain::MockChain;
use crate::types::packet_data::MockAnyPacketData;

define_components! {
    PacketDataTypes {
        AnyApp: MockAnyPacketData,
        IbcTransferApp: IbcTransferPacketData<MockChain, MockChain>,
        IbcTransferMintApp: IbcTransferMintPacketData<MockChain, MockChain>,
        IbcTransferUnescrowApp: IbcTransferUnescrowPacketData<MockChain>,
    }
}
