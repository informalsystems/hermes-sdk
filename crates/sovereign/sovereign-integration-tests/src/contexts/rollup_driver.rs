use cgp_core::HasComponents;
use hermes_sovereign_test_components::types::wallet::SovereignWallet;
use hermes_test_components::chain_driver::traits::types::address::ProvideAddressType;
use hermes_test_components::chain_driver::traits::types::wallet::ProvideWalletType;

pub struct SovereignRollupDriver {}

pub struct SovereignRollupDriverComponents;

impl HasComponents for SovereignRollupDriver {
    type Components = SovereignRollupDriverComponents;
}

impl ProvideAddressType<SovereignRollupDriver> for SovereignRollupDriverComponents {
    type Address = String;
}

impl ProvideWalletType<SovereignRollupDriver> for SovereignRollupDriverComponents {
    type Wallet = SovereignWallet;

    fn wallet_address(wallet: &SovereignWallet) -> &String {
        &wallet.address
    }
}
