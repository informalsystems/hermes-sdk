use cgp::prelude::*;
use hdpath::StandardHDPath;

#[cgp_component {
  name: WalletHdPathComponent,
  provider: WalletHdPathGetter,
  context: Bootstrap,
}]
pub trait HasWalletHdPath: Async {
    fn wallet_hd_path(&self) -> StandardHDPath;
}
