use cgp_core::HasErrorType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_sovereign_test_components::bootstrap::traits::build_rollup_driver::RollupDriverBuilder;
use hermes_sovereign_test_components::bootstrap::traits::types::rollup_driver::HasRollupDriverType;
use hermes_sovereign_test_components::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;
use hermes_sovereign_test_components::bootstrap::traits::types::rollup_node_config::HasRollupNodeConfigType;
use hermes_sovereign_test_components::types::rollup_genesis_config::SovereignGenesisConfig;
use hermes_sovereign_test_components::types::rollup_node_config::SovereignRollupNodeConfig;
use hermes_test_components::runtime::traits::types::child_process::HasChildProcessType;
use tokio::process::Child;

use crate::contexts::rollup_driver::SovereignRollupDriver;

pub struct BuildSovereignRollupDriver;

impl<Bootstrap, Runtime> RollupDriverBuilder<Bootstrap> for BuildSovereignRollupDriver
where
    Bootstrap: HasRuntimeType<Runtime = Runtime>
        + HasRollupDriverType<RollupDriver = SovereignRollupDriver>
        + HasRollupNodeConfigType<RollupNodeConfig = SovereignRollupNodeConfig>
        + HasRollupGenesisConfigType<RollupGenesisConfig = SovereignGenesisConfig>
        + HasErrorType,
    Runtime: HasChildProcessType<ChildProcess = Child>,
{
    async fn build_rollup_driver(
        _bootstrap: &Bootstrap,
        rollup_node_config: SovereignRollupNodeConfig,
        genesis_config: SovereignGenesisConfig,
        rollup_process: Child,
    ) -> Result<SovereignRollupDriver, Bootstrap::Error> {
        Ok(SovereignRollupDriver {
            rollup_node_config,
            genesis_config,
            rollup_process,
        })
    }
}
