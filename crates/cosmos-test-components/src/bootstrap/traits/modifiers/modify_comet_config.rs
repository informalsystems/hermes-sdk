use cgp_core::prelude::*;
use toml::Value;

#[async_trait]
pub trait CanModifyCometConfig: HasErrorType {
    fn modify_comet_config(&self, comet_config: &mut Value) -> Result<(), Self::Error>;
}
