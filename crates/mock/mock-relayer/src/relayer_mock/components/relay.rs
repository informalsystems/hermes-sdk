use cgp_core::prelude::*;
use hermes_logging_components::contexts::no_logger::ProvideNoLogger;
use hermes_logging_components::traits::has_logger::{GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent};
use hermes_relayer_components::relay::impls::message_senders::chain_sender::SendIbcMessagesToChain;
use hermes_relayer_components::relay::impls::message_senders::update_client::SendIbcMessagesWithUpdateClient;
use hermes_relayer_components::relay::impls::update_client::skip::SkipUpdateClient;
use hermes_relayer_components::relay::impls::update_client::wait::WaitUpdateClient;
use hermes_relayer_components::relay::impls::packet_relayers::ack::base_ack_packet::BaseAckPacketRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::FullCycleRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::receive::base_receive_packet::BaseReceivePacketRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::receive::skip_received_packet::SkipReceivedPacketRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::timeout_unordered::timeout_unordered_packet::BaseTimeoutUnorderedPacketRelayer;
use hermes_relayer_components::relay::traits::ibc_message_sender::{MainSink, IbcMessageSenderComponent};
use hermes_relayer_components::relay::traits::update_client_message_builder::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::relay::traits::packet_relayer::PacketRelayerComponent;
use hermes_relayer_components::relay::traits::packet_relayers::ack_packet::AckPacketRelayerComponent;
use hermes_relayer_components::relay::traits::packet_relayers::receive_packet::ReceivePacketRelayerComponnent;
use hermes_relayer_components::relay::traits::packet_relayers::timeout_unordered_packet::TimeoutUnorderedPacketRelayerComponent;

use crate::relayer_mock::base::impls::relay::MockBuildUpdateClientMessage;

pub struct MockRelayComponents;

delegate_components! {
    MockRelayComponents {
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideNoLogger,
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
