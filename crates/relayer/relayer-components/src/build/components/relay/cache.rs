use core::marker::PhantomData;

use cgp::core::error::HasErrorType;
use cgp::core::Async;
use hermes_runtime_components::traits::mutex::HasMutex;

use crate::build::traits::builders::relay_builder::RelayBuilder;
use crate::build::traits::cache::HasRelayCache;
use crate::multi::traits::chain_at::ChainIdAt;
use crate::multi::traits::relay_at::ClientIdAt;

pub struct BuildRelayWithCache<InBuilder>(pub PhantomData<InBuilder>);

impl<InBuilder, Build, Src: Async, Dst: Async> RelayBuilder<Build, Src, Dst>
    for BuildRelayWithCache<InBuilder>
where
    ChainIdAt<Build, Src>: Ord + Clone,
    ChainIdAt<Build, Dst>: Ord + Clone,
    ClientIdAt<Build, Src, Dst>: Ord + Clone,
    ClientIdAt<Build, Dst, Src>: Ord + Clone,
    Build: HasRelayCache<Src, Dst> + HasErrorType,
    InBuilder: RelayBuilder<Build, Src, Dst>,
    Build::Relay: Clone,
{
    async fn build_relay(
        build: &Build,
        index: PhantomData<(Src, Dst)>,
        src_chain_id: &ChainIdAt<Build, Src>,
        dst_chain_id: &ChainIdAt<Build, Dst>,
        src_client_id: &ClientIdAt<Build, Src, Dst>,
        dst_client_id: &ClientIdAt<Build, Dst, Src>,
    ) -> Result<Build::Relay, Build::Error> {
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
                index,
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
