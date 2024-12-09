use cgp::prelude::*;
use tendermint_rpc::Url;

#[cgp_component {
  provider: GrpcAddressGetter,
  context: Chain,
}]
pub trait HasGrpcAddress: Async {
    fn grpc_address(&self) -> &Url;
}
