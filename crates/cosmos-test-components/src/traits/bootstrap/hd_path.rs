use cgp_core::prelude::*;

#[derive_component(WalletHdPathComponent, WalletHdPathGetter<Bootstrap>)]
pub trait HasWalletHdPath: Async {
    fn wallet_hd_path(&self) -> &str;
}
