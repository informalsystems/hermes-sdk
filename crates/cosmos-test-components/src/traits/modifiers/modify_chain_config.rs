use cgp_core::prelude::*;

use crate::traits::types::chain_config::HasChainConfigType;

#[async_trait]
pub trait CanModifyChainConfig: HasChainConfigType + HasErrorType {
    async fn modify_chain_config(
        &self,
        chain_config: &mut Self::ChainConfig,
    ) -> Result<(), Self::Error>;
}
