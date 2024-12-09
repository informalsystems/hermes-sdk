use cgp::prelude::*;

#[cgp_component {
  name: BridgeConfigTypeComponent,
  provider: ProvideBridgeConfigType,
  context: Bootstrap,
}]
pub trait HasBridgeConfigType: Async {
    type BridgeConfig: Async;
}
