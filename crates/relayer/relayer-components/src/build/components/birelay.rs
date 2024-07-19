use crate::build::traits::builders::birelay_builder::BiRelayBuilder;
use crate::build::traits::builders::birelay_from_relay_builder::CanBuildBiRelayFromRelays;
use crate::build::traits::builders::relay_builder::CanBuildRelay;
use crate::multi::traits::chain_at::ChainIdAt;
use crate::multi::traits::relay_at::ClientIdAt;
use crate::multi::types::index::Twindex;

pub struct BuildBiRelayFromRelays;

impl<Build, const A: usize, const B: usize> BiRelayBuilder<Build, A, B> for BuildBiRelayFromRelays
where
    Build: CanBuildBiRelayFromRelays<A, B> + CanBuildRelay<A, B> + CanBuildRelay<B, A>,
{
    async fn build_birelay(
        build: &Build,
        chain_id_a: &ChainIdAt<Build, A>,
        chain_id_b: &ChainIdAt<Build, B>,
        client_id_a: &ClientIdAt<Build, A, B>,
        client_id_b: &ClientIdAt<Build, B, A>,
    ) -> Result<Build::BiRelay, Build::Error> {
        let relay_a_to_b = build
            .build_relay(
                Twindex::<A, B>,
                chain_id_a,
                chain_id_b,
                client_id_a,
                client_id_b,
            )
            .await?;

        let relay_b_to_a = build
            .build_relay(
                Twindex::<B, A>,
                chain_id_b,
                chain_id_a,
                client_id_b,
                client_id_a,
            )
            .await?;

        build
            .build_birelay_from_relays(relay_a_to_b, relay_b_to_a)
            .await
    }
}
