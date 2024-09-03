use core::marker::PhantomData;

use cgp::core::error::HasErrorType;
use hermes_runtime_components::traits::mutex::HasMutex;

use crate::build::traits::builders::chain_builder::ChainBuilder;
use crate::build::traits::cache::HasChainCache;
use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::multi::types::index::Index;

pub struct BuildChainWithCache<InBuilder>(pub PhantomData<InBuilder>);

impl<InBuilder, Build, Chain, const I: usize> ChainBuilder<Build, I>
    for BuildChainWithCache<InBuilder>
where
    Chain: HasChainIdType + Clone,
    Chain::ChainId: Ord + Clone,
    Build: HasChainCache<I, Chain = Chain> + HasErrorType,
    InBuilder: ChainBuilder<Build, I>,
{
    async fn build_chain(
        build: &Build,
        index: Index<I>,
        chain_id: &Chain::ChainId,
    ) -> Result<Chain, Build::Error> {
        let mut cache = Build::Runtime::acquire_mutex(build.chain_cache()).await;

        if let Some(chain) = cache.get(chain_id) {
            Ok(chain.clone())
        } else {
            let chain = InBuilder::build_chain(build, index, chain_id).await?;
            cache.insert(chain_id.clone(), chain.clone());

            Ok(chain)
        }
    }
}
