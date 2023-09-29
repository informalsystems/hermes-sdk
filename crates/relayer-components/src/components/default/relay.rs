use core::marker::PhantomData;

use cgp_core::delegate_component;

use crate::core::traits::run::RunnerComponent;
use crate::relay::components::auto_relayers::concurrent_bidirectional::ConcurrentBidirectionalRelayer;
use crate::relay::components::auto_relayers::concurrent_event::ConcurrentEventSubscriptionRelayer;
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

delegate_component!(
    IbcMessageSenderComponent<MainSink>,
    DefaultRelayComponents<BaseComponents>,
    SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>,
);

delegate_component!(
    UpdateClientMessageBuilderComponent,
    DefaultRelayComponents<BaseComponents>,
    SkipUpdateClient<WaitUpdateClient<BuildUpdateClientMessages>>,
);

delegate_component!(
    PacketRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    LockPacketRelayer<LoggerRelayer<FilterRelayer<FullCycleRelayer>>>,
);

delegate_component!(
    PacketFilterComponent,
    DefaultRelayComponents<BaseComponents>,
    BaseComponents,
);

delegate_component!(
    ReceivePacketRelayerComponnent,
    DefaultRelayComponents<BaseComponents>,
    SkipReceivedPacketRelayer<BaseReceivePacketRelayer>,
);

delegate_component!(
    AckPacketRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    BaseAckPacketRelayer,
);

delegate_component!(
    TimeoutUnorderedPacketRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    BaseTimeoutUnorderedPacketRelayer,
);

delegate_component!(
    EventRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    PacketEventRelayer,
);

delegate_component!(
    RunnerComponent,
    DefaultRelayComponents<BaseComponents>,
    ConcurrentBidirectionalRelayer<ConcurrentEventSubscriptionRelayer>,
);

delegate_component!(
    AutoRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    ConcurrentEventSubscriptionRelayer,
);

delegate_component!(
    ClientCreatorComponent,
    DefaultRelayComponents<BaseComponents>,
    CreateClientWithChains,
);

delegate_component!(
    PacketClearerComponent,
    DefaultRelayComponents<BaseComponents>,
    ClearReceivePackets,
);

delegate_component!(
    ChannelInitializerComponent,
    DefaultRelayComponents<BaseComponents>,
    InitializeChannel,
);

delegate_component!(
    ChannelOpenTryRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    RelayChannelOpenTry,
);

delegate_component!(
    ChannelOpenAckRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    RelayChannelOpenAck,
);

delegate_component!(
    ChannelOpenConfirmRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    RelayChannelOpenConfirm,
);

delegate_component!(
    ChannelOpenHandshakeRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    RelayChannelOpenHandshake,
);

delegate_component!(
    ConnectionOpenAckRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    RelayConnectionOpenAck,
);

delegate_component!(
    ConnectionOpenConfirmRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    RelayConnectionOpenConfirm,
);

delegate_component!(
    ConnectionInitializerComponent,
    DefaultRelayComponents<BaseComponents>,
    InitializeConnection,
);

delegate_component!(
    ConnectionOpenTryRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    RelayConnectionOpenTry,
);

delegate_component!(
    ConnectionOpenHandshakeRelayerComponent,
    DefaultRelayComponents<BaseComponents>,
    RelayConnectionOpenHandshake,
);
