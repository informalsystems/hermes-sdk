use hermes_logging_components::impls::IgnoreLog;
use hermes_logging_components::traits::LoggerComponent;
use hermes_prelude::*;
use hermes_relayer_components::relay::impls::{
    BaseAckPacketRelayer, BaseReceivePacketRelayer, BaseTimeoutUnorderedPacketRelayer,
    PerformFullRelay, SendIbcMessagesToChain, SendIbcMessagesWithUpdateClient, SkipReceivedPacket,
    SkipUpdateClient, WaitUpdateClient,
};
use hermes_relayer_components::relay::traits::{
    AckPacketRelayerComponent, IbcMessageSenderComponent, MainSink, PacketRelayerComponent,
    ReceivePacketRelayerComponent, TargetUpdateClientMessageBuilderComponent,
    TimeoutUnorderedPacketRelayerComponent,
};

use crate::relayer_mock::base::impls::relay::MockBuildUpdateClientMessage;
pub use crate::relayer_mock::contexts::relay::MockRelayComponents;

delegate_components! {
    MockRelayComponents {
        LoggerComponent: IgnoreLog,
        IbcMessageSenderComponent<MainSink>:
            SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>,
        PacketRelayerComponent: PerformFullRelay,
        ReceivePacketRelayerComponent:
            SkipReceivedPacket<BaseReceivePacketRelayer>,
        AckPacketRelayerComponent:
            BaseAckPacketRelayer,
        TimeoutUnorderedPacketRelayerComponent:
            BaseTimeoutUnorderedPacketRelayer,
        TargetUpdateClientMessageBuilderComponent:
            SkipUpdateClient<WaitUpdateClient<MockBuildUpdateClientMessage>>,
    }
}
