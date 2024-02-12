use hermes_test_components::chain_driver::traits::types::address::HasAddressType;
use hermes_test_components::chain_driver::traits::types::wallet::ProvideWalletType;

use crate::types::wallet::SovereignWallet;

pub struct ProvideSovereignWalletType;

impl<RollupDriver> ProvideWalletType<RollupDriver> for ProvideSovereignWalletType
where
    RollupDriver: HasAddressType<Address = String>,
{
    type Wallet = SovereignWallet;

    fn wallet_address(wallet: &SovereignWallet) -> &String {
        &wallet.address
    }
}
