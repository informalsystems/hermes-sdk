use alloc::boxed::Box;

use cgp_core::prelude::*;

use crate::traits::chain::types::chain::HasChainType;

#[derive_component(ChainBootstrapperComponent, ChainBootstrapper<Bootstrap>)]
#[async_trait]
pub trait CanBootstrapChain: HasChainType + HasErrorType {
    async fn bootstrap_chain(&self, chain_id_prefix: &str) -> Result<Self::Chain, Self::Error>;
}
