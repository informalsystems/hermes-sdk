use core::str::FromStr;

use cgp::prelude::*;
use hdpath::StandardHDPath;

use crate::bootstrap::traits::fields::hd_path::{WalletHdPathComponent, WalletHdPathGetter};

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
