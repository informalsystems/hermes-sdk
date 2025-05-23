use alloc::string::String;

use cgp::core::component::UseContext;
use hermes_prelude::*;

#[cgp_component {
  provider: ConfigUpdater,
  context: ChainDriver,
}]
pub trait CanUpdateConfig<Config>: HasAsyncErrorType {
    fn update_config(&self, config: &mut Config) -> Result<String, Self::Error>;
}

#[cgp_provider(ConfigUpdaterComponent)]
impl<ChainDriver, Config> ConfigUpdater<ChainDriver, Config> for UseFields
where
    ChainDriver: CanUpdateConfig<Config>,
{
    fn update_config(
        chain_driver: &ChainDriver,
        config: &mut Config,
    ) -> Result<String, ChainDriver::Error> {
        chain_driver.update_config(config)
    }
}
