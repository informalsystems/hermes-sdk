use cgp::prelude::*;

#[cgp_component {
  provider: GrpcPortGetter,
  context: ChainDriver,
}]
pub trait HasGrpcPort {
    fn grpc_port(&self) -> u16;
}
