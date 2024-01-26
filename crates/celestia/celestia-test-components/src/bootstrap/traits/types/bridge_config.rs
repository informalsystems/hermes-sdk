use cgp_core::prelude::*;

#[derive_component(BridgeConfigTypeComponent, ProvideBridgeConfigType<Bootstrap>)]
pub trait HasBridgeConfigType: Async {
    type BridgeConfig: Async;
}
