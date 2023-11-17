use cgp_core::Async;
use tendermint_rpc::client::CompatMode;

pub trait HasCometCompatMode: Async {
    fn comet_compat_mode(&self) -> &CompatMode;
}
