use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::num::TryFromIntError;
use core::time::Duration;

use cgp::core::error::ErrorOf;
use cgp::extra::runtime::HasRuntime;
use hermes_chain_components::traits::{
    CanAdjustHeight, CanQueryBlockTime, CanQueryChainHeight, CanSendMessages, HasHeightType,
    HeightOf,
};
use hermes_logging_components::traits::CanLog;
use hermes_prelude::*;
use hermes_runtime_components::traits::{CanRunConcurrentTasks, Task};

use crate::birelay::traits::{
    AutoBiRelayer, AutoBiRelayerComponent, HasBiRelayTypes, HasTwoWayRelay,
};
use crate::relay::traits::{
    CanAutoRelayWithHeights, CanRefreshClient, DestinationTarget, HasChainTargets, HasDstChain,
    HasRelayChains, HasSrcChain, SourceTarget,
};

pub struct LogAutoBiRelay<'a, BiRelay>
where
    BiRelay: HasBiRelayTypes<ChainA: HasHeightType, ChainB: HasHeightType>,
{
    pub bi_relay: &'a BiRelay,
    pub clear_past_blocks: &'a Option<Duration>,
    pub stop_after_blocks: &'a Option<Duration>,
    pub start_height_a: &'a HeightOf<BiRelay::ChainA>,
    pub start_height_b: &'a HeightOf<BiRelay::ChainB>,
    pub end_height_a: &'a Option<HeightOf<BiRelay::ChainA>>,
    pub end_height_b: &'a Option<HeightOf<BiRelay::ChainB>>,
}

#[cgp_new_provider(AutoBiRelayerComponent)]
impl<BiRelay> AutoBiRelayer<BiRelay> for PerformAutoBiRelay
where
    BiRelay: HasRuntime
        + HasTwoWayRelay
        + HasBiRelayTypes
        + for<'a> CanLog<LogAutoBiRelay<'a, BiRelay>>
        + CanRaiseAsyncError<TryFromIntError>
        + CanRaiseAsyncError<ErrorOf<BiRelay::ChainA>>
        + CanRaiseAsyncError<ErrorOf<BiRelay::ChainB>>,
    BiRelay::RelayAToB: Clone
        + HasSrcChain
        + HasDstChain
        + HasChainTargets
        + CanAutoRelayWithHeights<SourceTarget>
        + CanAutoRelayWithHeights<DestinationTarget>
        + CanRefreshClient<SourceTarget>
        + CanRefreshClient<DestinationTarget>,
    BiRelay::RelayBToA: Clone
        + HasRelayChains
        + HasChainTargets
        + CanAutoRelayWithHeights<SourceTarget>
        + CanAutoRelayWithHeights<DestinationTarget>
        + CanRefreshClient<SourceTarget>
        + CanRefreshClient<DestinationTarget>,
    BiRelay::ChainA: CanQueryChainHeight + CanQueryBlockTime + CanAdjustHeight + CanSendMessages,
    BiRelay::ChainB: CanQueryChainHeight + CanQueryBlockTime + CanAdjustHeight + CanSendMessages,
    BiRelay::Runtime: CanRunConcurrentTasks,
{
    async fn auto_bi_relay(
        bi_relay: &BiRelay,
        clear_past_blocks: Option<Duration>,
        stop_after_blocks: Option<Duration>,
        refresh_rate_a_to_b: Option<Duration>,
        refresh_rate_b_to_a: Option<Duration>,
    ) -> Result<(), BiRelay::Error> {
        let relay_a_to_b = bi_relay.relay_a_to_b();
        let relay_b_to_a = bi_relay.relay_b_to_a();

        let chain_a = relay_a_to_b.src_chain();
        let chain_b = relay_a_to_b.dst_chain();

        let block_time_a = chain_a
            .query_block_time()
            .await
            .map_err(BiRelay::raise_error)?;

        let block_time_b = chain_b
            .query_block_time()
            .await
            .map_err(BiRelay::raise_error)?;

        let height_a = chain_a
            .query_chain_height()
            .await
            .map_err(BiRelay::raise_error)?;

        let height_b = chain_b
            .query_chain_height()
            .await
            .map_err(BiRelay::raise_error)?;

        let end_height_a = if let Some(stop_after_blocks) = stop_after_blocks {
            let relative_height = (stop_after_blocks.as_millis() / block_time_a.as_millis())
                .try_into()
                .map_err(BiRelay::raise_error)?;

            Some(
                BiRelay::ChainA::add_height(&height_a, relative_height)
                    .map_err(BiRelay::raise_error)?,
            )
        } else {
            None
        };

        let end_height_b = if let Some(stop_after_blocks) = stop_after_blocks {
            let relative_height = (stop_after_blocks.as_millis() / block_time_b.as_millis())
                .try_into()
                .map_err(BiRelay::raise_error)?;

            Some(
                BiRelay::ChainB::add_height(&height_b, relative_height)
                    .map_err(BiRelay::raise_error)?,
            )
        } else {
            None
        };

        let start_height_a = if let Some(clear_past_blocks) = clear_past_blocks {
            let relative_height = (clear_past_blocks.as_millis() / block_time_a.as_millis())
                .try_into()
                .map_err(BiRelay::raise_error)?;

            BiRelay::ChainA::sub_height(&height_a, relative_height).map_err(BiRelay::raise_error)?
        } else {
            height_a
        };

        let start_height_b = if let Some(clear_past_blocks) = clear_past_blocks {
            let relative_height = (clear_past_blocks.as_millis() / block_time_b.as_millis())
                .try_into()
                .map_err(BiRelay::raise_error)?;

            BiRelay::ChainB::sub_height(&height_b, relative_height).map_err(BiRelay::raise_error)?
        } else {
            height_b
        };

        bi_relay
            .log(
                "starting auto bi-relaying",
                &LogAutoBiRelay {
                    bi_relay,
                    clear_past_blocks: &clear_past_blocks,
                    stop_after_blocks: &stop_after_blocks,
                    start_height_a: &start_height_a,
                    start_height_b: &start_height_b,
                    end_height_a: &end_height_a,
                    end_height_b: &end_height_b,
                },
            )
            .await;

        let tasks: Vec<Box<BiRelayTask<BiRelay>>> = vec![
            Box::new(BiRelayTask::SourceAToB {
                relay: relay_a_to_b.clone(),
                start_height: start_height_a.clone(),
                end_height: end_height_a.clone(),
                refresh_rate: refresh_rate_a_to_b,
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
                refresh_rate: refresh_rate_b_to_a,
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
        refresh_rate: Option<Duration>,
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
        refresh_rate: Option<Duration>,
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
        + CanAutoRelayWithHeights<DestinationTarget>
        + CanRefreshClient<SourceTarget>
        + CanRefreshClient<DestinationTarget>,
    BiRelay::RelayBToA: HasChainTargets
        + CanAutoRelayWithHeights<SourceTarget>
        + CanAutoRelayWithHeights<DestinationTarget>
        + CanRefreshClient<SourceTarget>
        + CanRefreshClient<DestinationTarget>,
{
    async fn run(self) {
        match self {
            BiRelayTask::SourceAToB {
                relay,
                start_height,
                end_height,
                refresh_rate,
            } => {
                let auto_relay_task =
                    relay.auto_relay_with_heights(SourceTarget, &start_height, end_height.as_ref());
                if let Some(refresh_rate) = refresh_rate {
                    let auto_refresh_task =
                        relay.auto_refresh_client(SourceTarget, refresh_rate, end_height.as_ref());

                    let _ = futures::join!(auto_relay_task, auto_refresh_task);
                } else {
                    let _ = auto_relay_task.await;
                }
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
                refresh_rate,
            } => {
                let auto_relay_task =
                    relay.auto_relay_with_heights(SourceTarget, &start_height, end_height.as_ref());

                if let Some(refresh_rate) = refresh_rate {
                    let auto_refresh_task =
                        relay.auto_refresh_client(SourceTarget, refresh_rate, end_height.as_ref());

                    let _ = futures::join!(auto_relay_task, auto_refresh_task);
                } else {
                    let _ = auto_relay_task.await;
                }
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
