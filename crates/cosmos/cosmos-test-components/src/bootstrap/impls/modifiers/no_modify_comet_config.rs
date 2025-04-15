use cgp::prelude::*;
use toml::Value;

use crate::bootstrap::traits::{CometConfigModifier, CometConfigModifierComponent};

pub struct NoModifyCometConfig;

#[cgp_provider(CometConfigModifierComponent)]
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
