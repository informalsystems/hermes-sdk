use cgp::prelude::*;

#[cgp_component {
  name: GrpcPortGetterComponent,
  provider: GrpcPortGetter,
  context: ChainDriver,
}]
pub trait HasGrpcPort {
    fn grpc_port(&self) -> u16;
}
