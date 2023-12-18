use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_core::RunnerComponent;

use crate::logger::traits::has_logger::{LoggerFieldComponent, LoggerTypeComponent};
use crate::relay::components::auto_relayers::both_targets::RelayBothTargets;
use crate::relay::components::auto_relayers::event::RelayEvents;
use crate::relay::components::create_client::CreateClientWithChains;
use crate::relay::components::event_relayers::packet_event::PacketEventRelayer;
use crate::relay::components::message_senders::chain_sender::SendIbcMessagesToChain;
use crate::relay::components::message_senders::update_client::SendIbcMessagesWithUpdateClient;
use crate::relay::components::packet_clearers::receive_packet::ClearReceivePackets;
use crate::relay::components::packet_relayers::ack::base_ack_packet::BaseAckPacketRelayer;
use crate::relay::components::packet_relayers::general::filter_relayer::FilterRelayer;
use crate::relay::components::packet_relayers::general::full_relay::FullCycleRelayer;
use crate::relay::components::packet_relayers::general::lock::LockPacketRelayer;
use crate::relay::components::packet_relayers::general::log::LoggerRelayer;
use crate::relay::components::packet_relayers::receive::base_receive_packet::BaseReceivePacketRelayer;
use crate::relay::components::packet_relayers::receive::skip_received_packet::SkipReceivedPacketRelayer;
use crate::relay::components::packet_relayers::timeout_unordered::timeout_unordered_packet::BaseTimeoutUnorderedPacketRelayer;
use crate::relay::components::update_client::build::BuildUpdateClientMessages;
use crate::relay::components::update_client::skip::SkipUpdateClient;
use crate::relay::components::update_client::wait::WaitUpdateClient;
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
use crate::relay::traits::channel::open_ack::ChannelOpenAckRelayerComponent;
use crate::relay::traits::channel::open_confirm::ChannelOpenConfirmRelayerComponent;
use crate::relay::traits::channel::open_handshake::ChannelOpenHandshakeRelayerComponent;
use crate::relay::traits::channel::open_init::ChannelInitializerComponent;
use crate::relay::traits::channel::open_try::ChannelOpenTryRelayerComponent;
use crate::relay::traits::components::auto_relayer::AutoRelayerComponent;
use crate::relay::traits::components::client_creator::ClientCreatorComponent;
use crate::relay::traits::components::event_relayer::EventRelayerComponent;
use crate::relay::traits::components::ibc_message_sender::{IbcMessageSenderComponent, MainSink};
use crate::relay::traits::components::packet_clearer::PacketClearerComponent;
use crate::relay::traits::components::packet_filter::PacketFilterComponent;
use crate::relay::traits::components::packet_relayer::PacketRelayerComponent;
use crate::relay::traits::components::packet_relayers::ack_packet::AckPacketRelayerComponent;
use crate::relay::traits::components::packet_relayers::receive_packet::ReceivePacketRelayerComponnent;
use crate::relay::traits::components::packet_relayers::timeout_unordered_packet::TimeoutUnorderedPacketRelayerComponent;
use crate::relay::traits::components::update_client_message_builder::UpdateClientMessageBuilderComponent;
use crate::relay::traits::connection::open_ack::ConnectionOpenAckRelayerComponent;
use crate::relay::traits::connection::open_confirm::ConnectionOpenConfirmRelayerComponent;
use crate::relay::traits::connection::open_handshake::ConnectionOpenHandshakeRelayerComponent;
use crate::relay::traits::connection::open_init::ConnectionInitializerComponent;
use crate::relay::traits::connection::open_try::ConnectionOpenTryRelayerComponent;

pub struct DefaultRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    DefaultRelayComponents<BaseComponents>;
    IbcMessageSenderComponent<MainSink>: SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>,
    UpdateClientMessageBuilderComponent: SkipUpdateClient<WaitUpdateClient<BuildUpdateClientMessages>>,
    PacketRelayerComponent: LockPacketRelayer<LoggerRelayer<FilterRelayer<FullCycleRelayer>>>,
    ReceivePacketRelayerComponnent: SkipReceivedPacketRelayer<BaseReceivePacketRelayer>,
    AckPacketRelayerComponent: BaseAckPacketRelayer,
    TimeoutUnorderedPacketRelayerComponent: BaseTimeoutUnorderedPacketRelayer,
    EventRelayerComponent: PacketEventRelayer,
    RunnerComponent: RelayBothTargets,
    AutoRelayerComponent: RelayEvents,
    ClientCreatorComponent: CreateClientWithChains,
    PacketClearerComponent: ClearReceivePackets,
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
    [
        ErrorTypeComponent,
        ErrorRaiserComponent,
        LoggerTypeComponent,
        LoggerFieldComponent,
        PacketFilterComponent,
    ]:
        BaseComponents,
);
