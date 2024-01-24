use cgp_core::prelude::*;

#[derive_component(GrpcPortGetterComponent, GrpcPortGetter<ChainDriver>)]
pub trait HasGrpcPort {
    fn grpc_port(&self) -> u16;
}
