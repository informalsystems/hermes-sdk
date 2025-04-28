use core::str::FromStr;

use hdpath::StandardHDPath;
use hermes_prelude::*;

use crate::bootstrap::traits::{WalletHdPathComponent, WalletHdPathGetter};

pub struct ProvideCosmosHdPath;

#[cgp_provider(WalletHdPathComponent)]
impl<Bootstrap> WalletHdPathGetter<Bootstrap> for ProvideCosmosHdPath
where
    Bootstrap: Async,
{
    fn wallet_hd_path(_bootstrap: &Bootstrap) -> StandardHDPath {
        StandardHDPath::from_str("m/44'/118'/0'/0/0").unwrap()
    }
}
