use cgp_core::prelude::*;
use hermes_relayer_components::logger::traits::has_logger::{LoggerTypeComponent, LoggerFieldComponent};
use hermes_relayer_components::relay::components::message_senders::chain_sender::SendIbcMessagesToChain;
use hermes_relayer_components::relay::components::message_senders::update_client::SendIbcMessagesWithUpdateClient;
use hermes_relayer_components::relay::components::update_client::skip::SkipUpdateClient;
use hermes_relayer_components::relay::components::update_client::wait::WaitUpdateClient;
use hermes_relayer_components::relay::components::packet_relayers::ack::base_ack_packet::BaseAckPacketRelayer;
use hermes_relayer_components::relay::components::packet_relayers::general::full_relay::FullCycleRelayer;
use hermes_relayer_components::relay::components::packet_relayers::receive::base_receive_packet::BaseReceivePacketRelayer;
use hermes_relayer_components::relay::components::packet_relayers::receive::skip_received_packet::SkipReceivedPacketRelayer;
use hermes_relayer_components::relay::components::packet_relayers::timeout_unordered::timeout_unordered_packet::BaseTimeoutUnorderedPacketRelayer;
use hermes_relayer_components::relay::traits::components::ibc_message_sender::{MainSink, IbcMessageSenderComponent};
use hermes_relayer_components::relay::traits::components::update_client_message_builder::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::relay::traits::components::packet_relayer::PacketRelayerComponent;
use hermes_relayer_components::relay::traits::components::packet_relayers::ack_packet::AckPacketRelayerComponent;
use hermes_relayer_components::relay::traits::components::packet_relayers::receive_packet::ReceivePacketRelayerComponnent;
use hermes_relayer_components::relay::traits::components::packet_relayers::timeout_unordered_packet::TimeoutUnorderedPacketRelayerComponent;
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;

use crate::relayer_mock::base::impls::relay::MockBuildUpdateClientMessage;

pub struct MockRelayComponents;

delegate_components! {
    MockRelayComponents {
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
        IbcMessageSenderComponent<MainSink>:
            SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>,
        PacketRelayerComponent: FullCycleRelayer,
        ReceivePacketRelayerComponnent:
            SkipReceivedPacketRelayer<BaseReceivePacketRelayer>,
        AckPacketRelayerComponent:
            BaseAckPacketRelayer,
        TimeoutUnorderedPacketRelayerComponent:
            BaseTimeoutUnorderedPacketRelayer,
        UpdateClientMessageBuilderComponent:
            SkipUpdateClient<WaitUpdateClient<MockBuildUpdateClientMessage>>,
    }
}
