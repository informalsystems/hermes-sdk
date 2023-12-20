use cgp_core::Async;
use http::Uri;

pub trait HasGrpcAddress: Async {
    fn grpc_address(&self) -> &Uri;
}
