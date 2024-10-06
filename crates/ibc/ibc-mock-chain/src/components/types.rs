use core::marker::PhantomData;

use alloc::string::String;
use cgp::core::error::ErrorTypeComponent;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::AddressTypeComponent;
use hermes_chain_type_components::traits::types::amount::AmountTypeComponent;
use hermes_chain_type_components::traits::types::denom::DenomTypeComponent;
use hermes_chain_type_components::traits::types::ibc::channel_id::ChannelIdTypeComponent;
use hermes_ibc_components::traits::types::app_id::AppIdTypeComponent;
use hermes_ibc_components::traits::types::message_header::IbcMessageHeaderTypeComponent;
use hermes_ibc_components::traits::types::packet::header::PacketHeaderTypeComponent;
use hermes_ibc_components::traits::types::packet::nonce::PacketNonceTypeComponent;
use hermes_ibc_components::traits::types::packet::packet::PacketTypeComponent;
use hermes_ibc_components::traits::types::packet::timeout::PacketTimeoutTypeComponent;
use hermes_ibc_components::traits::types::payload::data::PayloadDataTypeComponent;
use hermes_ibc_components::traits::types::payload::header::PayloadHeaderTypeComponent;
use hermes_ibc_components::types::any_app::AnyApp;
use hermes_ibc_components::types::message_header::IbcMessageHeader;
use hermes_ibc_components::types::packet::IbcPacket;
use hermes_ibc_components::types::packet_header::IbcPacketHeader;
use hermes_ibc_components::types::payload_header::IbcPayloadHeader;

use crate::contexts::chain::MockChain;
use crate::types::address::MockAddress;
use crate::types::amount::MockAmount;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;
use crate::types::denom::MockDenom;
use crate::types::height::MockHeight;
use crate::types::nonce::MockNonce;
use crate::types::packet_data::MockAnyPayloadData;
use crate::types::tagged::Tagged;

pub struct MockChainTypes<Chain, Counterparty>(pub PhantomData<(Chain, Counterparty)>);

delegate_components! {
    <A: Async, B: Async>
    MockChainTypes<A, B> {
        ErrorTypeComponent: String,
        AddressTypeComponent: Tagged<A, B, MockAddress>,
        DenomTypeComponent: Tagged<A, B, MockDenom>,
        AmountTypeComponent: Tagged<A, B, MockAmount>,
        AppIdTypeComponent: Tagged<A, B, MockAppId>,
        ChannelIdTypeComponent: Tagged<A, B, MockChannelId>,
        PacketNonceTypeComponent: Tagged<A, B, MockNonce>,
        PacketTimeoutTypeComponent: Tagged<A, B, MockHeight>,
        PacketTypeComponent: IbcPacket<Tagged<A, B, MockChain>, Tagged<B, A, MockChain>, AnyApp>,
        PacketHeaderTypeComponent: IbcPacketHeader<Tagged<A, B, MockChain>, Tagged<B, A, MockChain>>,
        PayloadHeaderTypeComponent: IbcPayloadHeader<Tagged<A, B, MockChain>, Tagged<B, A, MockChain>>,
        PayloadDataTypeComponent: MockAnyPayloadData<A, B>,
        IbcMessageHeaderTypeComponent: IbcMessageHeader<Tagged<A, B, MockChain>, Tagged<B, A, MockChain>>,
    }
}
