use cgp_core::HasComponents;
use hermes_sovereign_cosmos_relayer::contexts::sovereign_rollup::SovereignRollup;
use hermes_sovereign_test_components::types::rollup_genesis_config::SovereignGenesisConfig;
use hermes_sovereign_test_components::types::rollup_node_config::SovereignRollupNodeConfig;
use hermes_sovereign_test_components::types::wallet::SovereignWallet;
use hermes_test_components::chain_driver::traits::types::address::ProvideAddressType;
use hermes_test_components::chain_driver::traits::types::wallet::ProvideWalletType;
use tokio::process::Child;

pub struct SovereignRollupDriver {
    pub rollup: SovereignRollup,
    pub rollup_node_config: SovereignRollupNodeConfig,
    pub genesis_config: SovereignGenesisConfig,
    pub rollup_process: Child,
}

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
