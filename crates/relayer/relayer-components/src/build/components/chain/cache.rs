use core::marker::PhantomData;

use cgp_core::HasErrorType;
use hermes_runtime_components::traits::mutex::HasMutex;

use crate::build::traits::cache::HasChainCache;
use crate::build::traits::components::chain_builder::ChainBuilder;
use crate::build::traits::target::chain::ChainBuildTarget;
use crate::build::types::aliases::{TargetChain, TargetChainId};

pub struct BuildChainWithCache<InBuilder>(pub PhantomData<InBuilder>);

impl<InBuilder, Build, Target> ChainBuilder<Build, Target> for BuildChainWithCache<InBuilder>
where
    TargetChain<Build, Target>: Clone,
    TargetChainId<Build, Target>: Ord + Clone,
    Build: HasChainCache<Target> + HasErrorType,
    InBuilder: ChainBuilder<Build, Target>,
    Target: ChainBuildTarget<Build>,
{
    async fn build_chain(
        build: &Build,
        target: Target,
        chain_id: &TargetChainId<Build, Target>,
    ) -> Result<TargetChain<Build, Target>, Build::Error> {
        let mut cache = Build::Runtime::acquire_mutex(build.chain_cache()).await;

        if let Some(chain) = cache.get(chain_id) {
            Ok(chain.clone())
        } else {
            let chain = InBuilder::build_chain(build, target, chain_id).await?;
            cache.insert(chain_id.clone(), chain.clone());

            Ok(chain)
        }
    }
}
