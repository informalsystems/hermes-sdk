pub use cgp::extra::run::RunnerComponent;
use cgp::prelude::*;
use hermes_relayer_components::components::default::relay::DefaultRelayComponents;
use hermes_relayer_components::relay::impls::message_senders::chain_sender::SendIbcMessagesToChain;
use hermes_relayer_components::relay::impls::message_senders::update_client::SendIbcMessagesWithUpdateClient;
use hermes_relayer_components::relay::impls::packet_relayers::general::filter_relayer::FilterRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::FullCycleRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LockPacketRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::general::log::LoggerRelayer;
pub use hermes_relayer_components::relay::traits::auto_relayer::AutoRelayerComponent;
pub use hermes_relayer_components::relay::traits::channel::open_ack::ChannelOpenAckRelayerComponent;
pub use hermes_relayer_components::relay::traits::channel::open_confirm::ChannelOpenConfirmRelayerComponent;
pub use hermes_relayer_components::relay::traits::channel::open_handshake::ChannelOpenHandshakeRelayerComponent;
pub use hermes_relayer_components::relay::traits::channel::open_init::ChannelInitializerComponent;
pub use hermes_relayer_components::relay::traits::channel::open_try::ChannelOpenTryRelayerComponent;
pub use hermes_relayer_components::relay::traits::client_creator::ClientCreatorComponent;
pub use hermes_relayer_components::relay::traits::connection::open_ack::ConnectionOpenAckRelayerComponent;
pub use hermes_relayer_components::relay::traits::connection::open_confirm::ConnectionOpenConfirmRelayerComponent;
pub use hermes_relayer_components::relay::traits::connection::open_handshake::ConnectionOpenHandshakeRelayerComponent;
pub use hermes_relayer_components::relay::traits::connection::open_init::ConnectionInitializerComponent;
pub use hermes_relayer_components::relay::traits::connection::open_try::ConnectionOpenTryRelayerComponent;
pub use hermes_relayer_components::relay::traits::event_relayer::EventRelayerComponent;
pub use hermes_relayer_components::relay::traits::ibc_message_sender::{
    IbcMessageSenderComponent, MainSink,
};
pub use hermes_relayer_components::relay::traits::packet_clearer::PacketClearerComponent;
pub use hermes_relayer_components::relay::traits::packet_relayer::PacketRelayerComponent;
pub use hermes_relayer_components::relay::traits::packet_relayers::ack_packet::AckPacketRelayerComponent;
pub use hermes_relayer_components::relay::traits::packet_relayers::receive_packet::ReceivePacketRelayerComponent;
pub use hermes_relayer_components::relay::traits::packet_relayers::timeout_unordered_packet::TimeoutUnorderedPacketRelayerComponent;
pub use hermes_relayer_components::relay::traits::update_client_message_builder::TargetUpdateClientMessageBuilderComponent;

use crate::batch::impls::message_sender::SendMessagesToBatchWorker;
pub use crate::batch::types::sink::BatchWorkerSink;
use crate::relay::components::packet_relayers::retry::RetryRelayer;

define_components! {
    ExtraRelayComponents {
        IbcMessageSenderComponent<MainSink>: SendMessagesToBatchWorker,
        IbcMessageSenderComponent<BatchWorkerSink>:
            SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>,
        PacketRelayerComponent:
            LockPacketRelayer<LoggerRelayer<FilterRelayer<RetryRelayer<FullCycleRelayer>>>>,
        [
            TargetUpdateClientMessageBuilderComponent,
            ReceivePacketRelayerComponent,
            AckPacketRelayerComponent,
            TimeoutUnorderedPacketRelayerComponent,
            EventRelayerComponent,
            ClientCreatorComponent,
            PacketClearerComponent,
            ChannelInitializerComponent,
            ChannelOpenTryRelayerComponent,
            ChannelOpenAckRelayerComponent,
            ChannelOpenConfirmRelayerComponent,
            ChannelOpenHandshakeRelayerComponent,
            ConnectionOpenAckRelayerComponent,
            ConnectionOpenConfirmRelayerComponent,
            ConnectionInitializerComponent,
            ConnectionOpenTryRelayerComponent,
            ConnectionOpenHandshakeRelayerComponent,
            AutoRelayerComponent,
            RunnerComponent,
        ]:
            DefaultRelayComponents,
    }
}
