use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;

use crate::bootstrap::traits::types::chain_config::HasChainConfigType;
use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(ChainConfigInitializerComponent, ChainConfigInitializer<Boostrap>)]
#[async_trait]
pub trait CanInitChainConfig: HasChainConfigType + HasRuntime + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_chain_config(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
    ) -> Result<Self::ChainConfig, Self::Error>;
}
