use cgp::prelude::*;
use tendermint_rpc::Url;

#[derive_component(GrpcAddressGetterComponent, GrpcAddressGetter<Chain>)]
pub trait HasGrpcAddress: Async {
    fn grpc_address(&self) -> &Url;
}
