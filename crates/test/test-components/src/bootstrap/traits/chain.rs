use cgp::prelude::*;

use crate::driver::traits::types::chain_driver::HasChainDriverType;

#[derive_component(ChainBootstrapperComponent, ChainBootstrapper<Bootstrap>)]
#[async_trait]
pub trait CanBootstrapChain: HasChainDriverType + HasErrorType {
    async fn bootstrap_chain(
        &self,
        chain_id_prefix: &str,
    ) -> Result<Self::ChainDriver, Self::Error>;
}
