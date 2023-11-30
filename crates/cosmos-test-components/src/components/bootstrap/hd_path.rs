use core::str::FromStr;

use cgp_core::Async;
use hdpath::StandardHDPath;

use crate::traits::fields::hd_path::WalletHdPathGetter;

pub struct ProvideCosmosHdPath;

impl<Bootstrap> WalletHdPathGetter<Bootstrap> for ProvideCosmosHdPath
where
    Bootstrap: Async,
{
    fn wallet_hd_path(_bootstrap: &Bootstrap) -> StandardHDPath {
        StandardHDPath::from_str("m/44'/118'/0'/0/0").unwrap()
    }
}
