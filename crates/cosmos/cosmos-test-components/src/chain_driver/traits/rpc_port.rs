use cgp::prelude::*;

#[cgp_component {
  name: RpcPortGetterComponent,
  provider: RpcPortGetter,
  context: ChainDriver,
}]
pub trait HasRpcPort {
    fn rpc_port(&self) -> u16;
}
