use cgp_core::prelude::*;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk_legacy::LegacyCosmosSdkBootstrapComponents;
use hermes_cosmos_test_components::bootstrap::traits::fields::hd_path::WalletHdPathComponent;
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_chain_id::ChainIdGeneratorComponent;
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use hermes_cosmos_test_components::bootstrap::traits::initializers::init_chain_data::ChainDataInitializerComponent;
use hermes_cosmos_test_components::bootstrap::traits::initializers::init_chain_home_dir::ChainHomeDirInitializerComponent;

use crate::bootstrap::impls::generator::wallet_config::GenerateCelestiaWalletConfig;
use crate::bootstrap::impls::initializers::init_bridge_data::InitCelestiaBridgeData;

pub struct CelestiaBootstrapComponents;

delegate_components! {
    #[mark_component(IsLegacyCosmosSdkBootstrapComponent)]
    #[mark_delegate(DelegatesToLegacyToCosmosSdkBootstrapComponents)]
    CelestiaBootstrapComponents {
        ChainDataInitializerComponent:
            InitCelestiaBridgeData,
        WalletConfigGeneratorComponent:
            GenerateCelestiaWalletConfig,
        [
            ChainIdGeneratorComponent,
            ChainHomeDirInitializerComponent,
            WalletHdPathComponent,
            // WalletInitializerComponent,
            // ChainConfigInitializerComponent,
            // GenesisConfigInitializerComponent,
            // GenesisWalletAdderComponent,
            // ChainFullNodeStarterComponent,
            // ChainBootstrapperComponent,
        ]:
            LegacyCosmosSdkBootstrapComponents,
    }
}
