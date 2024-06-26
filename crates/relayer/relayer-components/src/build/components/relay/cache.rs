use core::marker::PhantomData;

use cgp_core::error::HasErrorType;
use hermes_runtime_components::traits::mutex::HasMutex;

use crate::build::traits::cache::HasRelayCache;
use crate::build::traits::components::relay_builder::RelayBuilder;
use crate::build::traits::target::relay::RelayBuildTarget;
use crate::build::types::aliases::{
    TargetDstChainId, TargetDstClientId, TargetRelay, TargetSrcChainId, TargetSrcClientId,
};

pub struct BuildRelayWithCache<InBuilder>(pub PhantomData<InBuilder>);

impl<InBuilder, Build, Target> RelayBuilder<Build, Target> for BuildRelayWithCache<InBuilder>
where
    TargetSrcChainId<Build, Target>: Ord + Clone,
    TargetDstChainId<Build, Target>: Ord + Clone,
    TargetSrcClientId<Build, Target>: Ord + Clone,
    TargetDstClientId<Build, Target>: Ord + Clone,
    TargetRelay<Build, Target>: Clone,
    Build: HasRelayCache<Target> + HasErrorType,
    InBuilder: RelayBuilder<Build, Target>,
    Target: RelayBuildTarget<Build>,
{
    async fn build_relay(
        build: &Build,
        target: Target,
        src_chain_id: &TargetSrcChainId<Build, Target>,
        dst_chain_id: &TargetDstChainId<Build, Target>,
        src_client_id: &TargetSrcClientId<Build, Target>,
        dst_client_id: &TargetDstClientId<Build, Target>,
    ) -> Result<TargetRelay<Build, Target>, Build::Error> {
        let relay_id = (
            src_chain_id.clone(),
            dst_chain_id.clone(),
            src_client_id.clone(),
            dst_client_id.clone(),
        );

        let mut cache = Build::Runtime::acquire_mutex(build.relay_cache()).await;

        if let Some(relay) = cache.get(&relay_id) {
            Ok(relay.clone())
        } else {
            let relay = InBuilder::build_relay(
                build,
                target,
                src_chain_id,
                dst_chain_id,
                src_client_id,
                dst_client_id,
            )
            .await?;
            cache.insert(relay_id, relay.clone());

            Ok(relay)
        }
    }
}
