use crate::build::traits::builders::chain_builder::CanBuildChain;
use crate::build::traits::builders::relay_builder::RelayBuilder;
use crate::build::traits::builders::relay_from_chains_builder::CanBuildRelayFromChains;
use crate::multi::traits::chain_at::ChainIdAt;
use crate::multi::traits::relay_at::ClientIdAt;
use crate::multi::types::index::{Index, Twindex};

pub struct BuildRelayFromChains;

impl<Build, const SRC: usize, const DST: usize> RelayBuilder<Build, SRC, DST>
    for BuildRelayFromChains
where
    Build: CanBuildChain<SRC> + CanBuildChain<DST> + CanBuildRelayFromChains<SRC, DST>,
{
    async fn build_relay(
        build: &Build,
        index: Twindex<SRC, DST>,
        src_chain_id: &ChainIdAt<Build, SRC>,
        dst_chain_id: &ChainIdAt<Build, DST>,
        src_client_id: &ClientIdAt<Build, SRC, DST>,
        dst_client_id: &ClientIdAt<Build, DST, SRC>,
    ) -> Result<Build::Relay, Build::Error> {
        let src_chain = build.build_chain(Index::<SRC>, src_chain_id).await?;

        let dst_chain = build.build_chain(Index::<DST>, dst_chain_id).await?;

        build
            .build_relay_from_chains(index, src_client_id, dst_client_id, src_chain, dst_chain)
            .await
    }
}
