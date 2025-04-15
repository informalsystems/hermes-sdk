use cgp::prelude::*;
use serde_json::Value;

use crate::bootstrap::traits::{CosmosGenesisConfigModifier, CosmosGenesisConfigModifierComponent};

pub struct NoModifyGenesisConfig;

#[cgp_provider(CosmosGenesisConfigModifierComponent)]
impl<Bootstrap> CosmosGenesisConfigModifier<Bootstrap> for NoModifyGenesisConfig
where
    Bootstrap: HasAsyncErrorType,
{
    fn modify_genesis_config(
        _bootstrap: &Bootstrap,
        _config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        Ok(())
    }
}
