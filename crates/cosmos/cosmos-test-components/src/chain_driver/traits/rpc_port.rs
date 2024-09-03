use cgp::prelude::*;

#[derive_component(RpcPortGetterComponent, RpcPortGetter<ChainDriver>)]
pub trait HasRpcPort {
    fn rpc_port(&self) -> u16;
}
