use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::types::height::{CanAdjustHeight, HeightOf};

use crate::birelay::traits::{AutoBiRelayer, AutoBiRelayerComponent, HasTwoWayRelay};
use crate::multi::traits::relay_at::{HasRelayTypeAt, RelayAt};
use crate::relay::traits::auto_relayer::CanAutoRelayWithHeights;
use crate::relay::traits::chains::{HasRelayChains, SrcChainOf};
use crate::relay::traits::target::{DestinationTarget, HasChainTargets, SourceTarget};

pub enum BiRelayTask<BiRelay>
where
    BiRelay: HasRelayTypeAt<Index<0>, Index<1>> + HasRelayTypeAt<Index<1>, Index<0>>,
{
    AToBAtSource {
        relay: RelayAt<BiRelay, Index<0>, Index<1>>,
    },
}

#[new_cgp_provider(AutoBiRelayerComponent)]
impl<BiRelay, RelayAToB, RelayBToA, ChainA, ChainB> AutoBiRelayer<BiRelay> for PerformAutoBiRelay
where
    BiRelay: HasTwoWayRelay
        + HasRelayTypeAt<Index<0>, Index<1>, Relay = RelayAToB>
        + HasRelayTypeAt<Index<1>, Index<0>, Relay = RelayBToA>
        + CanRaiseAsyncError<ChainA::Error>
        + CanRaiseAsyncError<ChainB::Error>,
    RelayAToB: HasRelayChains<SrcChain = ChainA, DstChain = ChainB>
        + HasChainTargets
        + CanAutoRelayWithHeights<SourceTarget>
        + CanAutoRelayWithHeights<DestinationTarget>,
    RelayBToA: HasRelayChains<SrcChain = ChainB, DstChain = ChainA>
        + HasChainTargets
        + CanAutoRelayWithHeights<SourceTarget>
        + CanAutoRelayWithHeights<DestinationTarget>,
    ChainA: CanQueryChainHeight + CanAdjustHeight,
    ChainB: CanQueryChainHeight + CanAdjustHeight,
{
    async fn auto_bi_relay(
        bi_relay: &BiRelay,
        clear_past_blocks: Option<u64>,
        stop_after_blocks: Option<u64>,
    ) -> Result<(), BiRelay::Error> {
        let relay_a_to_b = bi_relay.relay_a_to_b();
        let relay_b_to_a = bi_relay.relay_b_to_a();

        let chain_a = relay_a_to_b.src_chain();
        let chain_b = relay_a_to_b.dst_chain();

        let height_a = chain_a
            .query_chain_height()
            .await
            .map_err(BiRelay::raise_error)?;
        let height_b = chain_b
            .query_chain_height()
            .await
            .map_err(BiRelay::raise_error)?;

        let end_height_a = if let Some(stop_after_blocks) = stop_after_blocks {
            Some(ChainA::add_height(&height_a, stop_after_blocks).map_err(BiRelay::raise_error)?)
        } else {
            None
        };

        let end_height_b = if let Some(stop_after_blocks) = stop_after_blocks {
            Some(ChainB::add_height(&height_b, stop_after_blocks).map_err(BiRelay::raise_error)?)
        } else {
            None
        };

        let start_height_a = if let Some(clear_past_blocks) = clear_past_blocks {
            ChainA::sub_height(&height_a, clear_past_blocks).map_err(BiRelay::raise_error)?
        } else {
            height_a
        };

        let start_height_b = if let Some(clear_past_blocks) = clear_past_blocks {
            ChainB::sub_height(&height_b, clear_past_blocks).map_err(BiRelay::raise_error)?
        } else {
            height_b
        };

        Ok(())
    }
}
