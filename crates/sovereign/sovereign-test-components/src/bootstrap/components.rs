use cgp_core::prelude::*;

use crate::bootstrap::impls::bootstrap_rollup::BootstrapSovereignRollup;
use crate::bootstrap::impls::generate_rollup_genesis::GenerateSovereignGenesis;
use crate::bootstrap::impls::generate_rollup_wallets::GenerateSovereignRollupWallets;
use crate::bootstrap::impls::init_rollup_node_config::InitSovereignRollupNodeConfig;
use crate::bootstrap::impls::start_rollup::StartSovereignRollup;
use crate::bootstrap::impls::types::rollup_genesis_config::ProvideSovereignGenesisConfig;
use crate::bootstrap::impls::types::rollup_node_config::ProvideSovereignRollupNodeConfig;
use crate::bootstrap::impls::write_rollup_genesis::WriteSovereignGenesis;
use crate::bootstrap::traits::bootstrap_rollup::RollupBootstrapperComponent;
use crate::bootstrap::traits::generate_rollup_genesis::RollupGenesisGeneratorComponent;
use crate::bootstrap::traits::generate_rollup_wallets::RollupWalletGeneratorComponent;
use crate::bootstrap::traits::init_rollup_node_config::RollupNodeConfigInitializerComponent;
use crate::bootstrap::traits::start_rollup::RollupStarterComponent;
use crate::bootstrap::traits::types::rollup_genesis_config::RollupGenesisConfigTypeComponent;
use crate::bootstrap::traits::types::rollup_node_config::RollupNodeConfigTypeComponent;
use crate::bootstrap::traits::write_rollup_genesis::RollupGenesisWriterComponent;

pub struct SovereignBootstrapComponents;

delegate_components! {
    #[mark_component(IsSovereignBootstrapComponent)]
    SovereignBootstrapComponents {
        RollupNodeConfigTypeComponent:
            ProvideSovereignRollupNodeConfig,
        RollupGenesisConfigTypeComponent:
            ProvideSovereignGenesisConfig,
        RollupBootstrapperComponent:
            BootstrapSovereignRollup,
        RollupNodeConfigInitializerComponent:
            InitSovereignRollupNodeConfig,
        RollupWalletGeneratorComponent:
            GenerateSovereignRollupWallets,
        RollupGenesisGeneratorComponent:
            GenerateSovereignGenesis,
        RollupGenesisWriterComponent:
            WriteSovereignGenesis,
        RollupStarterComponent:
            StartSovereignRollup,
    }
}
