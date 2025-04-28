use hermes_prelude::*;

#[cgp_component {
  provider: BridgeRpcPortGetter,
  context: BridgeDriver,
}]
pub trait HasBridgeRpcPort: Async {
    fn bridge_rpc_port(&self) -> u16;
}
