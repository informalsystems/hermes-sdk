use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

use crate::bootstrap::traits::types::chain_config::HasChainConfigType;

#[derive_component(ChainConfigInitializerComponent, ChainConfigInitializer<Boostrap>)]
#[async_trait]
pub trait CanInitChainConfig: HasChainConfigType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_chain_config(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
    ) -> Result<Self::ChainConfig, Self::Error>;
}
