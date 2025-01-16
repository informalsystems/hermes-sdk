use alloc::string::String;

use cgp::core::component::UseContext;
use cgp::prelude::*;

#[cgp_component {
  provider: ConfigUpdater,
  context: ChainDriver,
}]
pub trait CanUpdateConfig<Config>: HasAsyncErrorType {
    fn update_config(&self, config: &mut Config) -> Result<String, Self::Error>;
}

impl<ChainDriver, Config> ConfigUpdater<ChainDriver, Config> for UseContext
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
