#[cgp::re_export_imports]
mod preset {
    use cgp::extra::run::RunnerComponent;
    use cgp::prelude::*;

    use crate::error::impls::retry::ReturnMaxRetry;
    use crate::error::traits::retry::MaxErrorRetryGetterComponent;
    use crate::relay::impls::auto_relayers::both_targets::RelayBothTargets;
    use crate::relay::impls::auto_relayers::poll_event::RelayWithPolledEvents;
    use crate::relay::impls::auto_relayers::starting_current_height::AutoRelayStartingCurrentHeight;
    use crate::relay::impls::channel::open_ack::RelayChannelOpenAck;
    use crate::relay::impls::channel::open_confirm::RelayChannelOpenConfirm;
    use crate::relay::impls::channel::open_handshake::RelayChannelOpenHandshake;
    use crate::relay::impls::channel::open_init::InitializeChannel;
    use crate::relay::impls::channel::open_try::RelayChannelOpenTry;
    use crate::relay::impls::connection::open_ack::RelayConnectionOpenAck;
    use crate::relay::impls::connection::open_confirm::RelayConnectionOpenConfirm;
    use crate::relay::impls::connection::open_handshake::RelayConnectionOpenHandshake;
    use crate::relay::impls::connection::open_init::InitializeConnection;
    use crate::relay::impls::connection::open_try::RelayConnectionOpenTry;
    use crate::relay::impls::create_client::CreateClientWithChains;
    use crate::relay::impls::event_relayers::packet_event::PacketEventRelayer;
    use crate::relay::impls::message_senders::chain_sender::SendIbcMessagesToChain;
    use crate::relay::impls::message_senders::update_client::SendIbcMessagesWithUpdateClient;
    use crate::relay::impls::packet_filters::chain::FilterRelayPacketWithChains;
    use crate::relay::impls::packet_lock::ProvidePacketLockWithMutex;
    use crate::relay::impls::packet_relayers::ack::base_ack_packet::BaseAckPacketRelayer;
    use crate::relay::impls::packet_relayers::general::default::DefaultPacketRelayer;
    use crate::relay::impls::packet_relayers::receive::base_receive_packet::BaseReceivePacketRelayer;
    use crate::relay::impls::packet_relayers::receive::skip_received_packet::SkipReceivedPacket;
    use crate::relay::impls::packet_relayers::skip_cleared::SkipClearedPacket;
    use crate::relay::impls::packet_relayers::timeout_unordered::timeout_unordered_packet::BaseTimeoutUnorderedPacketRelayer;
    use crate::relay::impls::update_client::default::DefaultTargetUpdateClientMessageBuilder;
    use crate::relay::traits::auto_relayer::{
        AutoRelayerWithHeightsComponent, TargetAutoRelayerComponent,
    };
    use crate::relay::traits::channel::open_ack::ChannelOpenAckRelayerComponent;
    use crate::relay::traits::channel::open_confirm::ChannelOpenConfirmRelayerComponent;
    use crate::relay::traits::channel::open_handshake::ChannelOpenHandshakeRelayerComponent;
    use crate::relay::traits::channel::open_init::ChannelInitializerComponent;
    use crate::relay::traits::channel::open_try::ChannelOpenTryRelayerComponent;
    use crate::relay::traits::client_creator::ClientCreatorComponent;
    use crate::relay::traits::connection::open_ack::ConnectionOpenAckRelayerComponent;
    use crate::relay::traits::connection::open_confirm::ConnectionOpenConfirmRelayerComponent;
    use crate::relay::traits::connection::open_handshake::ConnectionOpenHandshakeRelayerComponent;
    use crate::relay::traits::connection::open_init::ConnectionInitializerComponent;
    use crate::relay::traits::connection::open_try::ConnectionOpenTryRelayerComponent;
    use crate::relay::traits::event_relayer::EventRelayerComponent;
    use crate::relay::traits::ibc_message_sender::{IbcMessageSenderComponent, MainSink};
    use crate::relay::traits::packet_filter::RelayPacketFilterComponent;
    use crate::relay::traits::packet_lock::PacketLockComponent;
    use crate::relay::traits::packet_relayer::PacketRelayerComponent;
    use crate::relay::traits::packet_relayers::ack_packet::AckPacketRelayerComponent;
    use crate::relay::traits::packet_relayers::receive_packet::ReceivePacketRelayerComponent;
    use crate::relay::traits::packet_relayers::timeout_unordered_packet::TimeoutUnorderedPacketRelayerComponent;
    use crate::relay::traits::update_client_message_builder::TargetUpdateClientMessageBuilderComponent;

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
