use cgp_core::Async;

use crate::bootstrap::traits::types::rollup_node_config::ProvideRollupNodeConfigType;
use crate::types::rollup_node_config::SovereignRollupNodeConfig;

pub struct ProvideSovereignRollupNodeConfig;

impl<Bootstrap> ProvideRollupNodeConfigType<Bootstrap> for ProvideSovereignRollupNodeConfig
where
    Bootstrap: Async,
{
    type RollupNodeConfig = SovereignRollupNodeConfig;
}
