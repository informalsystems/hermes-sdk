use cgp::prelude::*;

#[cgp_component {
  name: BridgeDriverTypeComponent,
  provider: ProvideBridgeDriverType,
  context: Bootstrap,
}]
pub trait HasBridgeDriverType: Async {
    type BridgeDriver: Async;
}
