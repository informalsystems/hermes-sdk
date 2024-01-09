use cgp_core::{async_trait, HasErrorType};

use crate::birelay::traits::two_way::HasTwoWayRelay;
use crate::build::impls::bootstrap::relay::CanBootstrapRelay;
use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::components::birelay_builder::CanBuildBiRelay;
use crate::build::traits::target::relay::RelayAToBTarget;
use crate::build::types::aliases::{ChainA, ChainB, ChainIdA, ChainIdB};
use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::traits::types::create_client::HasCreateClientOptionsType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::relay::traits::chains::HasRelayChains;

#[async_trait]
pub trait CanBootstrapBiRelay: HasBiRelayType + HasErrorType
where
    ChainA<Self>: HasChainIdType + HasCreateClientOptionsType<ChainB<Self>>,
    ChainB<Self>: HasChainIdType + HasCreateClientOptionsType<ChainA<Self>>,
{
    async fn bootstrap_birelay(
        &self,
        chain_id_a: &ChainIdA<Self>,
        chain_id_b: &ChainIdB<Self>,
        payload_options_a: &<ChainA<Self> as HasCreateClientOptionsType<ChainB<Self>>>::CreateClientPayloadOptions,
        payload_options_b: &<ChainB<Self> as HasCreateClientOptionsType<ChainA<Self>>>::CreateClientPayloadOptions,
    ) -> Result<Self::BiRelay, Self::Error>;
}

#[async_trait]
impl<Build, BiRelay, ChainA, ChainB, Error> CanBootstrapBiRelay for Build
where
    Build: HasBiRelayType<BiRelay = BiRelay>
        + HasErrorType<Error = Error>
        + CanBuildBiRelay
        + CanBootstrapRelay<RelayAToBTarget>,
    BiRelay: HasTwoWayRelay<ChainA = ChainA, ChainB = ChainB>,
    ChainA: HasChainIdType
        + HasCreateClientOptionsType<ChainB>
        + HasIbcChainTypes<ChainB>
        + HasErrorType,
    ChainB: HasChainIdType
        + HasCreateClientOptionsType<ChainA>
        + HasIbcChainTypes<ChainA>
        + HasErrorType,
{
    async fn bootstrap_birelay(
        &self,
        chain_id_a: &ChainA::ChainId,
        chain_id_b: &ChainB::ChainId,
        payload_options_a: &ChainA::CreateClientPayloadOptions,
        payload_options_b: &ChainB::CreateClientPayloadOptions,
    ) -> Result<BiRelay, Error> {
        let relay_a_to_b = self
            .bootstrap_relay(
                RelayAToBTarget,
                chain_id_a,
                chain_id_b,
                payload_options_a,
                payload_options_b,
            )
            .await?;

        let bi_relay = self
            .build_birelay(
                chain_id_a,
                chain_id_b,
                relay_a_to_b.src_client_id(),
                relay_a_to_b.dst_client_id(),
            )
            .await?;

        Ok(bi_relay)
    }
}
