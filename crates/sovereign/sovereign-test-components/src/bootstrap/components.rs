use cgp_core::prelude::*;

use crate::bootstrap::impls::bootstrap_rollup::BootstrapSovereignRollup;
use crate::bootstrap::impls::generate_rollup_genesis::GenerateSovereignGenesis;
use crate::bootstrap::impls::generate_rollup_wallets::GenerateSovereignRollupWallets;
use crate::bootstrap::impls::init_rollup_config::InitSovereignRollupConfig;
use crate::bootstrap::impls::types::rollup_config::ProvideSovereignRollupConfig;
use crate::bootstrap::impls::types::rollup_genesis_config::ProvideSovereignGenesisConfig;
use crate::bootstrap::impls::write_rollup_genesis::WriteSovereignGenesis;
use crate::bootstrap::traits::bootstrap_rollup::RollupBootstrapperComponent;
use crate::bootstrap::traits::generate_rollup_genesis::RollupGenesisGeneratorComponent;
use crate::bootstrap::traits::generate_rollup_wallets::RollupWalletGeneratorComponent;
use crate::bootstrap::traits::init_rollup_config::RollupConfigInitializerComponent;
use crate::bootstrap::traits::types::rollup_config::RollupConfigTypeComponent;
use crate::bootstrap::traits::types::rollup_genesis_config::RollupGenesisConfigTypeComponent;
use crate::bootstrap::traits::write_rollup_genesis::RollupGenesisWriterComponent;

pub struct SovereignBootstrapComponents;

delegate_components! {
    #[mark_component(IsSovereignBootstrapComponent)]
    SovereignBootstrapComponents {
        RollupConfigTypeComponent:
            ProvideSovereignRollupConfig,
        RollupGenesisConfigTypeComponent:
            ProvideSovereignGenesisConfig,
        RollupBootstrapperComponent:
            BootstrapSovereignRollup,
        RollupConfigInitializerComponent:
            InitSovereignRollupConfig,
        RollupWalletGeneratorComponent:
            GenerateSovereignRollupWallets,
        RollupGenesisGeneratorComponent:
            GenerateSovereignGenesis,
        RollupGenesisWriterComponent:
            WriteSovereignGenesis,
    }
}
