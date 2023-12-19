use ibc_relayer_components::components::default::relay::{IsDefaultRelayComponent, DefaultRelayComponents};
use ibc_relayer_components::logger::traits::has_logger::{LoggerTypeComponent, LoggerFieldComponent};
use ibc_relayer_components::relay::components::message_senders::chain_sender::SendIbcMessagesToChain;
use ibc_relayer_components::relay::components::message_senders::update_client::SendIbcMessagesWithUpdateClient;
use ibc_relayer_components::relay::components::packet_filters::allow_all::AllowAll;
use ibc_relayer_components::relay::components::packet_relayers::general::full_relay::FullCycleRelayer;
use ibc_relayer_components::relay::components::update_client::skip::SkipUpdateClient;
use ibc_relayer_components::relay::components::update_client::wait::WaitUpdateClient;
use ibc_relayer_components::relay::components::packet_relayers::ack::base_ack_packet::BaseAckPacketRelayer;
use ibc_relayer_components::relay::components::packet_relayers::receive::base_receive_packet::BaseReceivePacketRelayer;
use ibc_relayer_components::relay::components::packet_relayers::receive::skip_received_packet::SkipReceivedPacketRelayer;
use ibc_relayer_components::relay::components::packet_relayers::timeout_unordered::timeout_unordered_packet::BaseTimeoutUnorderedPacketRelayer;
use ibc_relayer_components::relay::traits::components::ibc_message_sender::{IbcMessageSenderComponent, MainSink};
use ibc_relayer_components::relay::traits::components::packet_filter::PacketFilterComponent;
use ibc_relayer_components::relay::traits::components::packet_relayer::PacketRelayerComponent;
use ibc_relayer_components::relay::traits::components::packet_relayers::ack_packet::AckPacketRelayerComponent;
use ibc_relayer_components::relay::traits::components::packet_relayers::receive_packet::ReceivePacketRelayerComponnent;
use ibc_relayer_components::relay::traits::components::packet_relayers::timeout_unordered_packet::TimeoutUnorderedPacketRelayerComponent;
use ibc_relayer_components::relay::traits::components::update_client_message_builder::UpdateClientMessageBuilderComponent;
use cgp_core::{delegate_components, DelegateComponent};
use ibc_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use ibc_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;

use crate::impls::relay::MockCosmosBuildUpdateClientMessage;

pub struct MockCosmosRelayComponents;

impl<Component> DelegateComponent<Component> for MockCosmosRelayComponents
where
    Self: IsDefaultRelayComponent<Component>,
{
    type Delegate = DefaultRelayComponents;
}

delegate_components!(
    MockCosmosRelayComponents;
    PacketFilterComponent: AllowAll,
    RuntimeTypeComponent:
        ProvideTokioRuntimeType,
    [
        LoggerTypeComponent,
        LoggerFieldComponent,
    ]:
        ProvideTracingLogger,
);
