use hermes_prelude::*;
use toml::Value;

use crate::bootstrap::traits::{CosmosSdkConfigModifier, CosmosSdkConfigModifierComponent};

pub struct NoModifyCosmosSdkConfig;

#[cgp_provider(CosmosSdkConfigModifierComponent)]
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
