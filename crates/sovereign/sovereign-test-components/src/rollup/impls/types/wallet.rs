use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::wallet::ProvideWalletType;

use crate::types::wallet::SovereignWallet;

pub struct ProvideSovereignWalletType;

impl<Rollup> ProvideWalletType<Rollup> for ProvideSovereignWalletType
where
    Rollup: HasAddressType<Address = String>,
{
    type Wallet = SovereignWallet;

    fn wallet_address(wallet: &SovereignWallet) -> &String {
        &wallet.address.address
    }
}
