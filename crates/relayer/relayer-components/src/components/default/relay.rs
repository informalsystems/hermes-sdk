#[cgp::re_export_imports]
mod preset {
    use cgp::extra::run::RunnerComponent;
    use hermes_prelude::*;

    use crate::error::impls::retry::ReturnMaxRetry;
    use crate::error::traits::MaxErrorRetryGetterComponent;
    use crate::relay::impls::{
        AutoRelayStartingCurrentHeight, BaseAckPacketRelayer, BaseReceivePacketRelayer,
        BaseTimeoutUnorderedPacketRelayer, CreateClientWithChains, DefaultPacketRelayer,
        DefaultTargetUpdateClientMessageBuilder, FilterRelayPacketWithChains, InitializeChannel,
        InitializeConnection, PacketEventRelayer, ProvidePacketLockWithMutex,
        RefreshClientWithInterval, RelayBothTargets, RelayChannelOpenAck, RelayChannelOpenConfirm,
        RelayChannelOpenHandshake, RelayChannelOpenTry, RelayConnectionOpenAck,
        RelayConnectionOpenConfirm, RelayConnectionOpenHandshake, RelayConnectionOpenTry,
        RelayWithPolledEvents, SendIbcMessagesToChain, SendIbcMessagesWithUpdateClient,
        SkipClearedPacket, SkipReceivedPacket,
    };
    use crate::relay::traits::{
        AckPacketRelayerComponent, AutoRelayerWithHeightsComponent, ChannelInitializerComponent,
        ChannelOpenAckRelayerComponent, ChannelOpenConfirmRelayerComponent,
        ChannelOpenHandshakeRelayerComponent, ChannelOpenTryRelayerComponent,
        ClientCreatorComponent, ClientRefresherComponent, ConnectionInitializerComponent,
        ConnectionOpenAckRelayerComponent, ConnectionOpenConfirmRelayerComponent,
        ConnectionOpenHandshakeRelayerComponent, ConnectionOpenTryRelayerComponent,
        EventRelayerComponent, IbcMessageSenderComponent, MainSink, PacketLockComponent,
        PacketRelayerComponent, ReceivePacketRelayerComponent, RelayPacketFilterComponent,
        TargetAutoRelayerComponent, TargetUpdateClientMessageBuilderComponent,
        TimeoutUnorderedPacketRelayerComponent,
    };

    cgp_preset! {
        DefaultRelayPreset {
            IbcMessageSenderComponent<MainSink>: SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>,
            TargetUpdateClientMessageBuilderComponent: DefaultTargetUpdateClientMessageBuilder,
            PacketRelayerComponent: DefaultPacketRelayer,
            ReceivePacketRelayerComponent: SkipClearedPacket<SkipReceivedPacket<BaseReceivePacketRelayer>>,
            AckPacketRelayerComponent: SkipClearedPacket<BaseAckPacketRelayer>,
            TimeoutUnorderedPacketRelayerComponent: SkipClearedPacket<BaseTimeoutUnorderedPacketRelayer>,
            EventRelayerComponent: PacketEventRelayer,
            RunnerComponent: RelayBothTargets,
            TargetAutoRelayerComponent: AutoRelayStartingCurrentHeight,
            AutoRelayerWithHeightsComponent: RelayWithPolledEvents,
            ClientRefresherComponent: RefreshClientWithInterval,
            ClientCreatorComponent: CreateClientWithChains,
            ChannelInitializerComponent: InitializeChannel,
            ChannelOpenTryRelayerComponent: RelayChannelOpenTry,
            ChannelOpenAckRelayerComponent: RelayChannelOpenAck,
            ChannelOpenConfirmRelayerComponent: RelayChannelOpenConfirm,
            ChannelOpenHandshakeRelayerComponent: RelayChannelOpenHandshake,
            ConnectionOpenAckRelayerComponent: RelayConnectionOpenAck,
            ConnectionOpenConfirmRelayerComponent: RelayConnectionOpenConfirm,
            ConnectionInitializerComponent: InitializeConnection,
            ConnectionOpenTryRelayerComponent: RelayConnectionOpenTry,
            ConnectionOpenHandshakeRelayerComponent: RelayConnectionOpenHandshake,
            RelayPacketFilterComponent: FilterRelayPacketWithChains,
            MaxErrorRetryGetterComponent: ReturnMaxRetry<3>,
            PacketLockComponent: ProvidePacketLockWithMutex,
        }
    }
}
