use cgp_core::HasErrorType;
use serde_json::Value;

use crate::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifier;

pub struct NoModifyGenesisConfig;

impl<Bootstrap> CosmosGenesisConfigModifier<Bootstrap> for NoModifyGenesisConfig
where
    Bootstrap: HasErrorType,
{
    fn modify_genesis_config(
        _bootstrap: &Bootstrap,
        _config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        Ok(())
    }
}
