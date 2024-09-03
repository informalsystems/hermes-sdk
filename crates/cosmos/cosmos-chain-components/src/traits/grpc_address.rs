use cgp::prelude::*;
use http::Uri;

#[derive_component(GrpcAddressGetterComponent, GrpcAddressGetter<Chain>)]
pub trait HasGrpcAddress: Async {
    fn grpc_address(&self) -> &Uri;
}
