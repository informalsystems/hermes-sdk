use cgp_core::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;

use crate::bootstrap::impls::bootstrap_bridge::BootstrapCelestiaBridge;
use crate::bootstrap::impls::copy_bridge_key::CopyBridgeKey;
use crate::bootstrap::impls::generate_wallet_config::GenerateCelestiaWalletConfig;
use crate::bootstrap::impls::init_bridge_data::InitCelestiaBridgeData;
use crate::bootstrap::impls::types::bridge_config::ProvideTomlBridgeConfig;
use crate::bootstrap::impls::update_bridge_config::UpdateCelestiaBridgeConfig;
use crate::bootstrap::traits::bootstrap_bridge::BridgeBootstrapperComponent;
use crate::bootstrap::traits::import_bridge_key::BridgeKeyImporterComponent;
use crate::bootstrap::traits::init_bridge_config::BridgeConfigInitializerComponent;
use crate::bootstrap::traits::init_bridge_data::BridgeDataInitializerComponent;
use crate::bootstrap::traits::types::bridge_config::BridgeConfigTypeComponent;

pub struct CelestiaBootstrapComponents;

delegate_components! {
    CelestiaBootstrapComponents {
        BridgeDataInitializerComponent:
            InitCelestiaBridgeData,
        WalletConfigGeneratorComponent:
            GenerateCelestiaWalletConfig,
        BridgeBootstrapperComponent:
            BootstrapCelestiaBridge,
        BridgeKeyImporterComponent:
            CopyBridgeKey,
        BridgeConfigTypeComponent:
            ProvideTomlBridgeConfig,
        BridgeConfigInitializerComponent:
            UpdateCelestiaBridgeConfig,
    }
}
