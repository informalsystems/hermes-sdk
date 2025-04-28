use hermes_chain_type_components::traits::{
    AddressTypeProviderComponent, ChannelIdTypeComponent, ClientIdTypeComponent,
    HeightTypeProviderComponent, TimeTypeComponent,
};
use hermes_ibc_components::traits::types::app_id::AppIdTypeComponent;
use hermes_ibc_components::traits::types::packet::nonce::PacketNonceTypeComponent;
use hermes_ibc_components::traits::types::packet::timeout::PacketTimeoutTypeComponent;
use hermes_prelude::*;

use crate::types::address::MockAddress;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;
use crate::types::client_id::MockClientId;
use crate::types::height::MockHeight;
use crate::types::nonce::MockNonce;

pub struct MockIbcChainTypes;

delegate_components! {
    MockIbcChainTypes {
        TimeTypeComponent: MockHeight,
        HeightTypeProviderComponent: MockHeight,
        AddressTypeProviderComponent: MockAddress,
        AppIdTypeComponent: MockAppId,
        ClientIdTypeComponent: MockClientId,
        ChannelIdTypeComponent: MockChannelId,
        PacketNonceTypeComponent: MockNonce,
        PacketTimeoutTypeComponent: MockHeight,
    }
}
