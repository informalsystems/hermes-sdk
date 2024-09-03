use core::marker::PhantomData;

use cgp::core::error::HasErrorType;
use hermes_runtime_components::traits::mutex::HasMutex;

use crate::build::traits::builders::relay_builder::RelayBuilder;
use crate::build::traits::cache::HasRelayCache;
use crate::multi::traits::chain_at::ChainIdAt;
use crate::multi::traits::relay_at::ClientIdAt;
use crate::multi::types::index::Twindex;

pub struct BuildRelayWithCache<InBuilder>(pub PhantomData<InBuilder>);

impl<InBuilder, Build, const SRC: usize, const DST: usize> RelayBuilder<Build, SRC, DST>
    for BuildRelayWithCache<InBuilder>
where
    ChainIdAt<Build, SRC>: Ord + Clone,
    ChainIdAt<Build, DST>: Ord + Clone,
    ClientIdAt<Build, SRC, DST>: Ord + Clone,
    ClientIdAt<Build, DST, SRC>: Ord + Clone,
    Build: HasRelayCache<SRC, DST> + HasErrorType,
    InBuilder: RelayBuilder<Build, SRC, DST>,
    Build::Relay: Clone,
{
    async fn build_relay(
        build: &Build,
        index: Twindex<SRC, DST>,
        src_chain_id: &ChainIdAt<Build, SRC>,
        dst_chain_id: &ChainIdAt<Build, DST>,
        src_client_id: &ClientIdAt<Build, SRC, DST>,
        dst_client_id: &ClientIdAt<Build, DST, SRC>,
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
