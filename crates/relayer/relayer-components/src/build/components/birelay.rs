use cgp_core::prelude::*;

use crate::birelay::traits::two_way::HasTwoChainTypes;
use crate::build::traits::components::birelay_builder::BiRelayBuilder;
use crate::build::traits::components::birelay_from_relay_builder::CanBuildBiRelayFromRelays;
use crate::build::traits::components::relay_builder::CanBuildRelay;
use crate::build::traits::target::relay::{RelayAToBTarget, RelayBToATarget};
use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub struct BuildBiRelayFromRelays;

#[async_trait]
impl<Build, ChainA, ChainB> BiRelayBuilder<Build> for BuildBiRelayFromRelays
where
    Build:
        CanBuildBiRelayFromRelays + CanBuildRelay<RelayAToBTarget> + CanBuildRelay<RelayBToATarget>,
    Build::BiRelay: HasTwoChainTypes<ChainA = ChainA, ChainB = ChainB>,
    ChainA: HasIbcChainTypes<ChainB> + HasErrorType,
    ChainB: HasIbcChainTypes<ChainA> + HasErrorType,
{
    async fn build_birelay(
        build: &Build,
        chain_id_a: &ChainA::ChainId,
        chain_id_b: &ChainB::ChainId,
        client_id_a: &ChainA::ClientId,
        client_id_b: &ChainB::ClientId,
    ) -> Result<Build::BiRelay, Build::Error> {
        let relay_a_to_b = build
            .build_relay(
                RelayAToBTarget,
                chain_id_a,
                chain_id_b,
                client_id_a,
                client_id_b,
            )
            .await?;

        let relay_b_to_a = build
            .build_relay(
                RelayBToATarget,
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
