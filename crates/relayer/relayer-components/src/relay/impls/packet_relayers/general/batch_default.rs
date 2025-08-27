use hermes_chain_components::traits::{
    CanBuildPacketFromWriteAck, CanQueryPacketIsCleared, CanQueryPacketIsReceived,
    CanReadPacketFields,
};
use hermes_logging_components::traits::CanLog;
use hermes_logging_components::types::{LevelInfo, LevelWarn};
use hermes_prelude::*;

use crate::chain::traits::{CanQueryChainStatus, HasWriteAckEvent};
use crate::relay::impls::{
    BatchFilterRelayer, BatchLoggerRelayer, BatchSkipClearedPackets, PerformFullBatchRelay,
};
use crate::relay::traits::{
    BatchPacketsRelayer, BatchPacketsRelayerComponent, CanFilterRelayPackets,
    CanRelayBatchReceivePackets, CanRelayBatchTimeoutUnorderedPackets, HasRelayChains,
    HasRelayPacketType,
};

pub struct BatchDefaultPacketRelayer;

#[cgp_provider(BatchPacketsRelayerComponent)]
impl<Relay, SrcChain, DstChain> BatchPacketsRelayer<Relay> for BatchDefaultPacketRelayer
where
    Relay: CanRelayBatchReceivePackets
        + CanRelayBatchTimeoutUnorderedPackets
        + HasRelayPacketType
        + CanFilterRelayPackets
        + CanLog<LevelInfo>
        + CanLog<LevelWarn>
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + CanRaiseAsyncError<SrcChain::Error>
        + CanRaiseAsyncError<DstChain::Error>,
    SrcChain:
        CanQueryChainStatus + CanQueryPacketIsCleared<DstChain> + CanReadPacketFields<DstChain>,
    DstChain: CanQueryChainStatus
        + HasWriteAckEvent<SrcChain>
        + CanBuildPacketFromWriteAck<SrcChain>
        + CanQueryPacketIsReceived<SrcChain>,
{
    async fn relay_packets(relay: &Relay, packets: &[Relay::Packet]) -> Result<(), Relay::Error> {
        <BatchLoggerRelayer<BatchFilterRelayer<BatchSkipClearedPackets<PerformFullBatchRelay>>>>::relay_packets(
            relay, packets,
        )
        .await
    }
}
