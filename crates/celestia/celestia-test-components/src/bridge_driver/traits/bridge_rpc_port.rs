use cgp::prelude::*;

#[derive_component(BridgeRpcPortGetterComponent, BridgeRpcPortGetter<BridgeDriver>)]
pub trait HasBridgeRpcPort: Async {
    fn bridge_rpc_port(&self) -> u16;
}
