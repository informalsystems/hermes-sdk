use core::marker::PhantomData;

use cgp::core::Async;

use crate::build::traits::builders::chain_builder::CanBuildChain;
use crate::build::traits::builders::relay_builder::RelayBuilder;
use crate::build::traits::builders::relay_from_chains_builder::CanBuildRelayFromChains;
use crate::multi::traits::chain_at::ChainIdAt;
use crate::multi::traits::relay_at::ClientIdAt;

pub struct BuildRelayFromChains;

impl<Build, Src: Async, Dst: Async> RelayBuilder<Build, Src, Dst> for BuildRelayFromChains
where
    Build: CanBuildChain<Src> + CanBuildChain<Dst> + CanBuildRelayFromChains<Src, Dst>,
{
    async fn build_relay(
        build: &Build,
        index: PhantomData<(Src, Dst)>,
        src_chain_id: &ChainIdAt<Build, Src>,
        dst_chain_id: &ChainIdAt<Build, Dst>,
        src_client_id: &ClientIdAt<Build, Src, Dst>,
        dst_client_id: &ClientIdAt<Build, Dst, Src>,
    ) -> Result<Build::Relay, Build::Error> {
        let src_chain = build.build_chain(PhantomData::<Src>, src_chain_id).await?;

        let dst_chain = build.build_chain(PhantomData::<Dst>, dst_chain_id).await?;

        build
            .build_relay_from_chains(index, src_client_id, dst_client_id, src_chain, dst_chain)
            .await
    }
}
