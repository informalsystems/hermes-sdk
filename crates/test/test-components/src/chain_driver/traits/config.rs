use alloc::string::String;

use cgp_core::prelude::*;

#[derive_component(ConfigUpdaterComponent, ConfigUpdater<ChainDriver>)]
pub trait CanUpdateConfig<Config>: HasErrorType {
    fn update_config(&self, config: &mut Config) -> Result<String, Self::Error>;
}
