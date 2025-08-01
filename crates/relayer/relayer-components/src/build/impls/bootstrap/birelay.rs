use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::Async;
use hermes_prelude::*;

use crate::build::impls::bootstrap::relay::CanBootstrapRelay;
use crate::build::traits::builders::birelay_builder::CanBuildBiRelay;
use crate::chain::traits::{
    CreateClientMessageOptionsOf, CreateClientPayloadOptionsOf, HasChainIdType,
    HasCreateClientMessageOptionsType, HasCreateClientPayloadOptionsType, HasIbcChainTypes,
};
use crate::multi::traits::birelay_at::HasBiRelayTypeAt;
use crate::multi::traits::chain_at::{ChainAt, ChainIdAt, HasChainTypeAt};
use crate::multi::traits::relay_at::HasBoundedRelayTypeAt;
use crate::relay::traits::{HasDstClientId, HasRelayClientIds, HasSrcClientId};

#[async_trait]
pub trait CanBootstrapBiRelay<A, B>:
    HasBiRelayTypeAt<A, B>
    + HasChainTypeAt<
        A,
        Chain: HasChainIdType
                   + HasCreateClientPayloadOptionsType<ChainAt<Self, B>>
                   + HasCreateClientMessageOptionsType<ChainAt<Self, B>>,
    > + HasChainTypeAt<
        B,
        Chain: HasChainIdType
                   + HasCreateClientPayloadOptionsType<ChainAt<Self, A>>
                   + HasCreateClientMessageOptionsType<ChainAt<Self, A>>,
    > + HasAsyncErrorType
{
    async fn bootstrap_birelay(
        &self,
        chain_id_a: &ChainIdAt<Self, A>,
        chain_id_b: &ChainIdAt<Self, B>,
        payload_options_a: &CreateClientPayloadOptionsOf<ChainAt<Self, A>, ChainAt<Self, B>>,
        payload_options_b: &CreateClientPayloadOptionsOf<ChainAt<Self, B>, ChainAt<Self, A>>,
        message_options_a: &CreateClientMessageOptionsOf<ChainAt<Self, A>, ChainAt<Self, B>>,
        message_options_b: &CreateClientMessageOptionsOf<ChainAt<Self, B>, ChainAt<Self, A>>,
        refresh_rate_a: Option<Duration>,
        refresh_rate_b: Option<Duration>,
    ) -> Result<Self::BiRelay, Self::Error>;
}

impl<Build, ChainA, ChainB, A, B> CanBootstrapBiRelay<A, B> for Build
where
    Build: Async
        + HasBiRelayTypeAt<A, B>
        + HasBoundedRelayTypeAt<A, B, Relay: HasRelayClientIds>
        + HasBoundedRelayTypeAt<B, A, Relay: HasRelayClientIds>
        + HasChainTypeAt<A, Chain = ChainA>
        + HasChainTypeAt<B, Chain = ChainB>
        + CanBuildBiRelay<A, B>
        + CanBootstrapRelay<A, B>,
    ChainA: Async
        + HasChainIdType
        + HasCreateClientPayloadOptionsType<ChainB>
        + HasCreateClientMessageOptionsType<ChainB>
        + HasIbcChainTypes<ChainB>
        + HasAsyncErrorType,
    ChainB: Async
        + HasChainIdType
        + HasCreateClientPayloadOptionsType<ChainA>
        + HasCreateClientMessageOptionsType<ChainA>
        + HasIbcChainTypes<ChainA>
        + HasAsyncErrorType,
{
    async fn bootstrap_birelay(
        &self,
        chain_id_a: &ChainA::ChainId,
        chain_id_b: &ChainB::ChainId,
        payload_options_a: &ChainA::CreateClientPayloadOptions,
        payload_options_b: &ChainB::CreateClientPayloadOptions,
        message_options_a: &ChainA::CreateClientMessageOptions,
        message_options_b: &ChainB::CreateClientMessageOptions,
        refresh_rate_a: Option<Duration>,
        refresh_rate_b: Option<Duration>,
    ) -> Result<Build::BiRelay, Build::Error> {
        let relay_a_to_b = self
            .bootstrap_relay(
                PhantomData,
                chain_id_a,
                chain_id_b,
                payload_options_a,
                payload_options_b,
                message_options_a,
                message_options_b,
                refresh_rate_a,
                refresh_rate_b,
            )
            .await?;

        let bi_relay = self
            .build_birelay(
                chain_id_a,
                chain_id_b,
                relay_a_to_b.src_client_id(),
                relay_a_to_b.dst_client_id(),
                refresh_rate_a,
                refresh_rate_b,
            )
            .await?;

        Ok(bi_relay)
    }
}
