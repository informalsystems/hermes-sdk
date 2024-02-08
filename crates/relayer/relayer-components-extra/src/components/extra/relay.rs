use cgp_core::prelude::*;
use cgp_core::RunnerComponent;
use hermes_relayer_components::components::default::relay::DefaultRelayComponents;
use hermes_relayer_components::relay::impls::message_senders::chain_sender::SendIbcMessagesToChain;
use hermes_relayer_components::relay::impls::message_senders::update_client::SendIbcMessagesWithUpdateClient;
use hermes_relayer_components::relay::impls::packet_relayers::general::filter_relayer::FilterRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::FullCycleRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LockPacketRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::general::log::LoggerRelayer;
use hermes_relayer_components::relay::traits::auto_relayer::AutoRelayerComponent;
use hermes_relayer_components::relay::traits::channel::open_ack::ChannelOpenAckRelayerComponent;
use hermes_relayer_components::relay::traits::channel::open_confirm::ChannelOpenConfirmRelayerComponent;
use hermes_relayer_components::relay::traits::channel::open_handshake::ChannelOpenHandshakeRelayerComponent;
use hermes_relayer_components::relay::traits::channel::open_init::ChannelInitializerComponent;
use hermes_relayer_components::relay::traits::channel::open_try::ChannelOpenTryRelayerComponent;
use hermes_relayer_components::relay::traits::client_creator::ClientCreatorComponent;
use hermes_relayer_components::relay::traits::connection::open_ack::ConnectionOpenAckRelayerComponent;
use hermes_relayer_components::relay::traits::connection::open_confirm::ConnectionOpenConfirmRelayerComponent;
use hermes_relayer_components::relay::traits::connection::open_handshake::ConnectionOpenHandshakeRelayerComponent;
use hermes_relayer_components::relay::traits::connection::open_init::ConnectionInitializerComponent;
use hermes_relayer_components::relay::traits::connection::open_try::ConnectionOpenTryRelayerComponent;
use hermes_relayer_components::relay::traits::event_relayer::EventRelayerComponent;
use hermes_relayer_components::relay::traits::ibc_message_sender::{
    IbcMessageSenderComponent, MainSink,
};
use hermes_relayer_components::relay::traits::packet_clearer::PacketClearerComponent;
use hermes_relayer_components::relay::traits::packet_relayer::PacketRelayerComponent;
use hermes_relayer_components::relay::traits::packet_relayers::ack_packet::AckPacketRelayerComponent;
use hermes_relayer_components::relay::traits::packet_relayers::receive_packet::ReceivePacketRelayerComponnent;
use hermes_relayer_components::relay::traits::packet_relayers::timeout_unordered_packet::TimeoutUnorderedPacketRelayerComponent;
use hermes_relayer_components::relay::traits::update_client_message_builder::UpdateClientMessageBuilderComponent;

use crate::batch::components::message_sender::SendMessagesToBatchWorker;
use crate::batch::types::sink::BatchWorkerSink;
use crate::relay::components::packet_relayers::retry::RetryRelayer;

pub struct ExtraRelayComponents;

delegate_components! {
    #[mark_component(IsExtraRelayComponent)]
    #[mark_delegate(DelegatesToExtraRelayComponents)]
    ExtraRelayComponents {
        IbcMessageSenderComponent<MainSink>: SendMessagesToBatchWorker,
        IbcMessageSenderComponent<BatchWorkerSink>:
            SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>,
        PacketRelayerComponent:
            LockPacketRelayer<LoggerRelayer<FilterRelayer<RetryRelayer<FullCycleRelayer>>>>,
        [
            UpdateClientMessageBuilderComponent,
            ReceivePacketRelayerComponnent,
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
