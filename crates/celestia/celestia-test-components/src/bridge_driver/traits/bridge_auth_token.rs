use cgp_core::prelude::*;

#[derive_component(BridgeAuthTokenTypeComponent, ProvideBridgeAuthTokenType<BridgeDriver>)]
pub trait HasBridgeAuthTokenType: Async {
    type BridgeAuthToken: Async;
}

pub type BridgeAuthTokenOf<BridgeDriver> =
    <BridgeDriver as HasBridgeAuthTokenType>::BridgeAuthToken;

#[derive_component(BridgeAuthTokenGetterComponent, BridgeAuthTokenGetter<BridgeDriver>)]
pub trait HasBridgeAuthToken: HasBridgeAuthTokenType {
    fn bridge_auth_token(&self) -> &Self::BridgeAuthToken;
}
