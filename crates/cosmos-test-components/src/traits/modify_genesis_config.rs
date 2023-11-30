use cgp_core::HasErrorType;

use crate::traits::types::genesis_config::HasGenesisConfigType;

pub trait CanModifyGenesisConfig: HasGenesisConfigType + HasErrorType {
    fn modify_genesis_config(&self, config: &mut Self::GenesisConfig) -> Result<(), Self::Error>;
}
