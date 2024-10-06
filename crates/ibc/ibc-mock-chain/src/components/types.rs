use core::marker::PhantomData;

use alloc::string::String;
use cgp::core::error::ErrorTypeComponent;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::AddressTypeComponent;
use hermes_chain_type_components::traits::types::amount::AmountTypeComponent;
use hermes_chain_type_components::traits::types::denom::DenomTypeComponent;
use hermes_chain_type_components::traits::types::ibc::channel_id::ChannelIdTypeComponent;
use hermes_ibc_components::traits::types::app_id::AppIdTypeComponent;
use hermes_ibc_components::traits::types::packet::nonce::PacketNonceTypeComponent;
use hermes_ibc_components::traits::types::packet::timeout::PacketTimeoutTypeComponent;

use crate::types::address::MockAddress;
use crate::types::amount::MockAmount;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;
use crate::types::denom::MockDenom;
use crate::types::height::MockHeight;
use crate::types::nonce::MockNonce;
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
    }
}
