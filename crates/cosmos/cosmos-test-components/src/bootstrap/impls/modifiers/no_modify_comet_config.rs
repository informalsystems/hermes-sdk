use cgp::core::error::HasAsyncErrorType;
use toml::Value;

use crate::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifier;

pub struct NoModifyCometConfig;

impl<Bootstrap> CometConfigModifier<Bootstrap> for NoModifyCometConfig
where
    Bootstrap: HasAsyncErrorType,
{
    fn modify_comet_config(
        _bootstrap: &Bootstrap,
        _comet_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        Ok(())
    }
}
