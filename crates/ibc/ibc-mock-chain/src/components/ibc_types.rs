use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::AddressTypeComponent;
use hermes_chain_type_components::traits::types::amount::AmountTypeComponent;
use hermes_chain_type_components::traits::types::denom::DenomTypeComponent;
use hermes_chain_type_components::traits::types::height::HeightTypeComponent;
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

define_components! {
    MockIbcChainTypes {
        HeightTypeComponent: MockHeight,
        AddressTypeComponent: MockAddress,
        DenomTypeComponent: MockDenom,
        AmountTypeComponent: MockAmount,
        AppIdTypeComponent: MockAppId,
        ChannelIdTypeComponent: MockChannelId,
        PacketNonceTypeComponent: MockNonce,
        PacketTimeoutTypeComponent: MockHeight,
    }
}
