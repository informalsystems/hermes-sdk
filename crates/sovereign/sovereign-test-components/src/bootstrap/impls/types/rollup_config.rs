use cgp_core::Async;

use crate::bootstrap::traits::types::rollup_config::ProvideRollupConfigType;
use crate::types::rollup_config::SovereignRollupConfig;

pub struct ProvideSovereignRollupConfig;

impl<Bootstrap> ProvideRollupConfigType<Bootstrap> for ProvideSovereignRollupConfig
where
    Bootstrap: Async,
{
    type RollupConfig = SovereignRollupConfig;
}
