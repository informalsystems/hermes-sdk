use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::AddressTypeComponent;
use hermes_chain_type_components::traits::types::height::HeightTypeComponent;
use hermes_chain_type_components::traits::types::ibc::channel_id::ChannelIdTypeComponent;
use hermes_chain_type_components::traits::types::quantity::QuantityTypeComponent;
use hermes_ibc_components::traits::types::app_id::AppIdTypeComponent;
use hermes_ibc_components::traits::types::packet::nonce::PacketNonceTypeComponent;
use hermes_ibc_components::traits::types::packet::timeout::PacketTimeoutTypeComponent;

use crate::types::address::MockAddress;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;
use crate::types::height::MockHeight;
use crate::types::nonce::MockNonce;
use crate::types::quantity::MockQuantity;

define_components! {
    MockIbcChainTypes {
        HeightTypeComponent: MockHeight,
        AddressTypeComponent: MockAddress,
        QuantityTypeComponent: MockQuantity,
        AppIdTypeComponent: MockAppId,
        ChannelIdTypeComponent: MockChannelId,
        PacketNonceTypeComponent: MockNonce,
        PacketTimeoutTypeComponent: MockHeight,
    }
}
