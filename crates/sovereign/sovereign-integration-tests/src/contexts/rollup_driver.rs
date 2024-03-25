use alloc::collections::BTreeMap;

use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use hermes_cosmos_relayer::types::error::{DebugError, ProvideCosmosError};
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeTypeComponent;
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::{
    RollupGetter, RollupTypeComponent,
};
use hermes_sovereign_relayer::contexts::sovereign_rollup::SovereignRollup;
use hermes_sovereign_test_components::types::rollup_genesis_config::SovereignGenesisConfig;
use hermes_sovereign_test_components::types::rollup_node_config::SovereignRollupNodeConfig;
use hermes_sovereign_test_components::types::wallet::SovereignWallet;
use tokio::process::Child;

use crate::impls::rollup::ProvideSovereignRollupType;

pub struct SovereignRollupDriver {
    pub rollup: SovereignRollup,
    pub node_config: SovereignRollupNodeConfig,
    pub genesis_config: SovereignGenesisConfig,
    pub wallets: BTreeMap<String, SovereignWallet>,
    pub rollup_process: Child,
}

pub struct SovereignRollupDriverComponents;

impl HasComponents for SovereignRollupDriver {
    type Components = SovereignRollupDriverComponents;
}

delegate_components! {
    SovereignRollupDriverComponents {
        ErrorTypeComponent:
            ProvideCosmosError,
        ErrorRaiserComponent:
            DebugError,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        RollupTypeComponent: ProvideSovereignRollupType,
    }
}

impl RollupGetter<SovereignRollupDriver> for SovereignRollupDriverComponents {
    fn rollup(rollup_driver: &SovereignRollupDriver) -> &SovereignRollup {
        &rollup_driver.rollup
    }
}
