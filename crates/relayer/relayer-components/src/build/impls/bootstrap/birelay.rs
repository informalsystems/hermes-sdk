use cgp_core::prelude::{async_trait, HasErrorType};

use crate::birelay::traits::two_way::HasTwoWayRelay;
use crate::build::impls::bootstrap::relay::CanBootstrapRelay;
use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::builders::birelay_builder::CanBuildBiRelay;
use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadOptionsType,
};
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::multi::types::index::Twindex;
use crate::relay::traits::chains::HasRelayChains;

#[async_trait]
pub trait CanBootstrapBiRelay: HasBiRelayType + HasErrorType
where
    ChainA<Self>: HasChainIdType
        + HasCreateClientPayloadOptionsType<ChainB<Self>>
        + HasCreateClientMessageOptionsType<ChainB<Self>>,
    ChainB<Self>: HasChainIdType
        + HasCreateClientPayloadOptionsType<ChainA<Self>>
        + HasCreateClientMessageOptionsType<ChainA<Self>>,
{
    async fn bootstrap_birelay(
        &self,
        chain_id_a: &ChainIdA<Self>,
        chain_id_b: &ChainIdB<Self>,
        payload_options_a: &<ChainA<Self> as HasCreateClientPayloadOptionsType<ChainB<Self>>>::CreateClientPayloadOptions,
        payload_options_b: &<ChainB<Self> as HasCreateClientPayloadOptionsType<ChainA<Self>>>::CreateClientPayloadOptions,
        message_options_a: &<ChainA<Self> as HasCreateClientMessageOptionsType<ChainB<Self>>>::CreateClientMessageOptions,
        message_options_b: &<ChainB<Self> as HasCreateClientMessageOptionsType<ChainA<Self>>>::CreateClientMessageOptions,
    ) -> Result<Self::BiRelay, Self::Error>;
}

impl<Build, BiRelay, ChainA, ChainB, Error> CanBootstrapBiRelay for Build
where
    Build: HasBiRelayType<BiRelay = BiRelay>
        + HasErrorType<Error = Error>
        + CanBuildBiRelay<0, 1>
        + CanBootstrapRelay<0, 1>,
    BiRelay: HasTwoWayRelay<ChainA = ChainA, ChainB = ChainB>,
    ChainA: HasChainIdType
        + HasCreateClientPayloadOptionsType<ChainB>
        + HasCreateClientMessageOptionsType<ChainB>
        + HasIbcChainTypes<ChainB>
        + HasErrorType,
    ChainB: HasChainIdType
        + HasCreateClientPayloadOptionsType<ChainA>
        + HasCreateClientMessageOptionsType<ChainA>
        + HasIbcChainTypes<ChainA>
        + HasErrorType,
{
    async fn bootstrap_birelay(
        &self,
        chain_id_a: &ChainA::ChainId,
        chain_id_b: &ChainB::ChainId,
        payload_options_a: &ChainA::CreateClientPayloadOptions,
        payload_options_b: &ChainB::CreateClientPayloadOptions,
        message_options_a: &ChainA::CreateClientMessageOptions,
        message_options_b: &ChainB::CreateClientMessageOptions,
    ) -> Result<BiRelay, Error> {
        let relay_a_to_b = self
            .bootstrap_relay(
                Twindex::<0, 1>,
                chain_id_a,
                chain_id_b,
                payload_options_a,
                payload_options_b,
                message_options_a,
                message_options_b,
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
