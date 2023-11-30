use cgp_core::prelude::*;

use crate::traits::types::chain_config::HasChainConfigType;
use crate::traits::types::file_path::HasFilePathType;

#[derive_component(ChainConfigInitializerComponent, ChainConfigInitializer<Boostrap>)]
#[async_trait]
pub trait CanInitChainConfig: HasChainConfigType + HasFilePathType + HasErrorType {
    async fn init_chain_config(
        &self,
        chain_home_dir: &Self::FilePath,
    ) -> Result<Self::ChainConfig, Self::Error>;
}
