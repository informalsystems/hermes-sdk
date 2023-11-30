use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

#[async_trait]
pub trait CanGenerateChainId: HasChainIdType {
    async fn generate_chain_id(&self, chain_id_prefix: &str) -> &Self::ChainId;
}
