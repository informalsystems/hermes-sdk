use cgp_core::Async;

use crate::bootstrap::traits::types::genesis_config::ProvideGenesisConfigType;

pub struct ProvideJsonGenesisConfigType;

impl<Bootstrap> ProvideGenesisConfigType<Bootstrap> for ProvideJsonGenesisConfigType
where
    Bootstrap: Async,
{
    type GenesisConfig = serde_json::Value;
}
