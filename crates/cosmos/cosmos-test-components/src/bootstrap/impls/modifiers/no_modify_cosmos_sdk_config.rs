use cgp::core::error::HasAsyncErrorType;
use toml::Value;

use crate::bootstrap::traits::modifiers::modify_cosmos_sdk_config::CosmosSdkConfigModifier;

pub struct NoModifyCosmosSdkConfig;

impl<Bootstrap> CosmosSdkConfigModifier<Bootstrap> for NoModifyCosmosSdkConfig
where
    Bootstrap: HasAsyncErrorType,
{
    fn modify_cosmos_sdk_config(
        _bootstrap: &Bootstrap,
        _cosmos_sdk_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        Ok(())
    }
}
