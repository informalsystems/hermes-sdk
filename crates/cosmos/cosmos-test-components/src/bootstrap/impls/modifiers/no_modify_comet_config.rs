use cgp_core::HasErrorType;
use toml::Value;

use crate::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifier;

pub struct NoModifyCometConfig;

impl<Bootstrap> CometConfigModifier<Bootstrap> for NoModifyCometConfig
where
    Bootstrap: HasErrorType,
{
    fn modify_comet_config(
        _bootstrap: &Bootstrap,
        _comet_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        Ok(())
    }
}
