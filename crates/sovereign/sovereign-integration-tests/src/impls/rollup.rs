use cgp_core::Async;
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::ProvideRollupType;
use hermes_sovereign_relayer::contexts::sovereign_rollup::SovereignRollup;

pub struct ProvideSovereignRollupType;

impl<ChainDriver> ProvideRollupType<ChainDriver> for ProvideSovereignRollupType
where
    ChainDriver: Async,
{
    type Rollup = SovereignRollup;
}
