use cgp::prelude::*;
use hdpath::StandardHDPath;

#[derive_component(WalletHdPathComponent, WalletHdPathGetter<Bootstrap>)]
pub trait HasWalletHdPath: Async {
    fn wallet_hd_path(&self) -> StandardHDPath;
}
