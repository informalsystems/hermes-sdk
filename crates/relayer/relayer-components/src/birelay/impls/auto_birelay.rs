use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use cgp::core::error::ErrorOf;
use cgp::extra::runtime::HasRuntime;
use cgp::prelude::*;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::types::height::{CanAdjustHeight, HasHeightType, HeightOf};
use hermes_runtime_components::traits::task::{CanRunConcurrentTasks, Task};

use crate::birelay::traits::{
    AutoBiRelayer, AutoBiRelayerComponent, HasBiRelayTypes, HasTwoWayRelay,
};
use crate::relay::traits::auto_relayer::CanAutoRelayWithHeights;
use crate::relay::traits::chains::{HasDstChain, HasRelayChains, HasSrcChain};
use crate::relay::traits::target::{DestinationTarget, HasChainTargets, SourceTarget};

#[new_cgp_provider(AutoBiRelayerComponent)]
impl<BiRelay> AutoBiRelayer<BiRelay> for PerformAutoBiRelay
where
    BiRelay: HasRuntime
        + HasTwoWayRelay
        + HasBiRelayTypes
        + CanRaiseAsyncError<ErrorOf<BiRelay::ChainA>>
        + CanRaiseAsyncError<ErrorOf<BiRelay::ChainB>>,
    BiRelay::RelayAToB: Clone
        + HasSrcChain
        + HasDstChain
        + HasChainTargets
        + CanAutoRelayWithHeights<SourceTarget>
        + CanAutoRelayWithHeights<DestinationTarget>,
    BiRelay::RelayBToA: Clone
        + HasRelayChains
        + HasChainTargets
        + CanAutoRelayWithHeights<SourceTarget>
        + CanAutoRelayWithHeights<DestinationTarget>,
    BiRelay::ChainA: CanQueryChainHeight + CanAdjustHeight,
    BiRelay::ChainB: CanQueryChainHeight + CanAdjustHeight,
    BiRelay::Runtime: CanRunConcurrentTasks,
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
            Some(
                BiRelay::ChainA::add_height(&height_a, stop_after_blocks)
                    .map_err(BiRelay::raise_error)?,
            )
        } else {
            None
        };

        let end_height_b = if let Some(stop_after_blocks) = stop_after_blocks {
            Some(
                BiRelay::ChainB::add_height(&height_b, stop_after_blocks)
                    .map_err(BiRelay::raise_error)?,
            )
        } else {
            None
        };

        let start_height_a = if let Some(clear_past_blocks) = clear_past_blocks {
            BiRelay::ChainA::sub_height(&height_a, clear_past_blocks)
                .map_err(BiRelay::raise_error)?
        } else {
            height_a
        };

        let start_height_b = if let Some(clear_past_blocks) = clear_past_blocks {
            BiRelay::ChainB::sub_height(&height_b, clear_past_blocks)
                .map_err(BiRelay::raise_error)?
        } else {
            height_b
        };

        let tasks: Vec<Box<BiRelayTask<BiRelay>>> = vec![
            Box::new(BiRelayTask::SourceAToB {
                relay: relay_a_to_b.clone(),
                start_height: start_height_a.clone(),
                end_height: end_height_a.clone(),
            }),
            Box::new(BiRelayTask::DestinationAToB {
                relay: relay_a_to_b.clone(),
                start_height: start_height_b.clone(),
                end_height: end_height_b.clone(),
            }),
            Box::new(BiRelayTask::SourceBToA {
                relay: relay_b_to_a.clone(),
                start_height: start_height_b,
                end_height: end_height_b,
            }),
            Box::new(BiRelayTask::DestinationBToA {
                relay: relay_b_to_a.clone(),
                start_height: start_height_a,
                end_height: end_height_a,
            }),
        ];

        bi_relay.runtime().run_concurrent_tasks(tasks).await;

        Ok(())
    }
}

pub enum BiRelayTask<BiRelay>
where
    BiRelay: HasBiRelayTypes<ChainA: HasHeightType, ChainB: HasHeightType>,
{
    SourceAToB {
        relay: BiRelay::RelayAToB,
        start_height: HeightOf<BiRelay::ChainA>,
        end_height: Option<HeightOf<BiRelay::ChainA>>,
    },
    DestinationAToB {
        relay: BiRelay::RelayAToB,
        start_height: HeightOf<BiRelay::ChainB>,
        end_height: Option<HeightOf<BiRelay::ChainB>>,
    },
    SourceBToA {
        relay: BiRelay::RelayBToA,
        start_height: HeightOf<BiRelay::ChainB>,
        end_height: Option<HeightOf<BiRelay::ChainB>>,
    },
    DestinationBToA {
        relay: BiRelay::RelayBToA,
        start_height: HeightOf<BiRelay::ChainA>,
        end_height: Option<HeightOf<BiRelay::ChainA>>,
    },
}

impl<BiRelay> Task for BiRelayTask<BiRelay>
where
    BiRelay: HasBiRelayTypes,
    BiRelay::RelayAToB: HasChainTargets
        + CanAutoRelayWithHeights<SourceTarget>
        + CanAutoRelayWithHeights<DestinationTarget>,
    BiRelay::RelayBToA: HasChainTargets
        + CanAutoRelayWithHeights<SourceTarget>
        + CanAutoRelayWithHeights<DestinationTarget>,
{
    async fn run(self) {
        match self {
            BiRelayTask::SourceAToB {
                relay,
                start_height,
                end_height,
            } => {
                let _ = relay
                    .auto_relay_with_heights(SourceTarget, &start_height, end_height.as_ref())
                    .await;
            }
            BiRelayTask::DestinationAToB {
                relay,
                start_height,
                end_height,
            } => {
                let _ = relay
                    .auto_relay_with_heights(DestinationTarget, &start_height, end_height.as_ref())
                    .await;
            }
            BiRelayTask::SourceBToA {
                relay,
                start_height,
                end_height,
            } => {
                let _ = relay
                    .auto_relay_with_heights(SourceTarget, &start_height, end_height.as_ref())
                    .await;
            }
            BiRelayTask::DestinationBToA {
                relay,
                start_height,
                end_height,
            } => {
                let _ = relay
                    .auto_relay_with_heights(DestinationTarget, &start_height, end_height.as_ref())
                    .await;
            }
        }
    }
}
