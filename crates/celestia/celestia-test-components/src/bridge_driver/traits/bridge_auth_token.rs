use cgp::prelude::*;

#[cgp_component {
  name: BridgeAuthTokenTypeComponent,
  provider: ProvideBridgeAuthTokenType,
  context: BridgeDriver,
}]
pub trait HasBridgeAuthTokenType: Async {
    type BridgeAuthToken: Async;
}

pub type BridgeAuthTokenOf<BridgeDriver> =
    <BridgeDriver as HasBridgeAuthTokenType>::BridgeAuthToken;

#[cgp_component {
  provider: BridgeAuthTokenGetter,
  context: BridgeDriver,
}]
pub trait HasBridgeAuthToken: HasBridgeAuthTokenType {
    fn bridge_auth_token(&self) -> &Self::BridgeAuthToken;
}
