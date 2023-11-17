use cgp_core::Async;

use crate::traits::bootstrap::hd_path::WalletHdPathGetter;

pub struct ProvideCosmosHdPath;

impl<Bootstrap> WalletHdPathGetter<Bootstrap> for ProvideCosmosHdPath
where
    Bootstrap: Async,
{
    fn wallet_hd_path(_bootstrap: &Bootstrap) -> &str {
        "m/44'/118'/0'/0/0"
    }
}
