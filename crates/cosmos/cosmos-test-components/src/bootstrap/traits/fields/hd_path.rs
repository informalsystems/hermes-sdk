use hdpath::StandardHDPath;
use hermes_prelude::*;

#[cgp_component {
  name: WalletHdPathComponent,
  provider: WalletHdPathGetter,
  context: Bootstrap,
}]
pub trait HasWalletHdPath: Async {
    fn wallet_hd_path(&self) -> StandardHDPath;
}
