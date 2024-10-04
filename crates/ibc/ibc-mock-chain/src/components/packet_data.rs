use core::marker::PhantomData;

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
use crate::types::tagged::Tagged;

pub struct PacketDataTypes<Chain, Counterparty>(pub PhantomData<(Chain, Counterparty)>);

delegate_components! {
    <A: Async, B: Async>
    PacketDataTypes<A, B> {
        AnyApp: MockAnyPacketData<A, B>,
        IbcTransferApp: IbcTransferPacketData<Tagged<A, B, MockChain>, Tagged<B, A, MockChain>>,
        IbcTransferMintApp: IbcTransferMintPacketData<Tagged<A, B, MockChain>, Tagged<B, A, MockChain>>,
        IbcTransferUnescrowApp: IbcTransferUnescrowPacketData<Tagged<B, A, MockChain>>,
    }
}
