use core::marker::PhantomData;

use cgp::prelude::*;

use crate::build::traits::builders::chain_builder::{ChainBuilder, ChainBuilderComponent};
use crate::build::traits::cache::HasChainCache;
use crate::chain::traits::types::chain_id::HasChainIdType;

pub struct BuildChainWithCache<InBuilder>(pub PhantomData<InBuilder>);

#[cgp_provider(ChainBuilderComponent)]
impl<InBuilder, Build, Chain, I: Async> ChainBuilder<Build, I> for BuildChainWithCache<InBuilder>
where
    Chain: HasChainIdType + Clone,
    Chain::ChainId: Ord + Clone,
    Build: HasChainCache<I, Chain = Chain> + HasAsyncErrorType,
    InBuilder: ChainBuilder<Build, I>,
{
    async fn build_chain(
        build: &Build,
        index: PhantomData<I>,
        chain_id: &Chain::ChainId,
    ) -> Result<Chain, Build::Error> {
        let mut cache = build.chain_cache().lock().await;

        if let Some(chain) = cache.get(chain_id) {
            Ok(chain.clone())
        } else {
            let chain = InBuilder::build_chain(build, index, chain_id).await?;
            cache.insert(chain_id.clone(), chain.clone());

            Ok(chain)
        }
    }
}
