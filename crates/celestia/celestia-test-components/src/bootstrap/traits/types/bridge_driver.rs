use cgp_core::prelude::*;

#[derive_component(BridgeDriverTypeComponent, ProvideBridgeDriverType<Bootstrap>)]
pub trait HasBridgeDriverType: Async {
    type BridgeDriver: Async;
}
