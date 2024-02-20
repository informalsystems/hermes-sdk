use alloc::collections::BTreeMap;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_sovereign_client_components::sovereign::traits::chain::rollup::RollupGetter;
use hermes_sovereign_client_components::sovereign::traits::chain::rollup::RollupTypeComponent;
use hermes_sovereign_cosmos_relayer::contexts::sovereign_rollup::SovereignRollup;
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
            ProvideEyreError,
        ErrorRaiserComponent:
            RaiseDebugError,
        RuntimeTypeComponent:
            ProvideTokioRuntimeType,
        RollupTypeComponent: ProvideSovereignRollupType,
    }
}

impl RollupGetter<SovereignRollupDriver> for SovereignRollupDriverComponents {
    fn rollup(rollup_driver: &SovereignRollupDriver) -> &SovereignRollup {
        &rollup_driver.rollup
    }
}
