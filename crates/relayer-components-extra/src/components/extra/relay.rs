use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::RunnerComponent;
use ibc_relayer_components::components::default::relay::DefaultRelayComponents;
use ibc_relayer_components::relay::components::message_senders::chain_sender::SendIbcMessagesToChain;
use ibc_relayer_components::relay::components::message_senders::update_client::SendIbcMessagesWithUpdateClient;
use ibc_relayer_components::relay::components::packet_relayers::general::filter_relayer::FilterRelayer;
use ibc_relayer_components::relay::components::packet_relayers::general::full_relay::FullCycleRelayer;
use ibc_relayer_components::relay::components::packet_relayers::general::lock::LockPacketRelayer;
use ibc_relayer_components::relay::components::packet_relayers::general::log::LoggerRelayer;
use ibc_relayer_components::relay::traits::channel::open_ack::ChannelOpenAckRelayerComponent;
use ibc_relayer_components::relay::traits::channel::open_confirm::ChannelOpenConfirmRelayerComponent;
use ibc_relayer_components::relay::traits::channel::open_handshake::ChannelOpenHandshakeRelayerComponent;
use ibc_relayer_components::relay::traits::channel::open_init::ChannelInitializerComponent;
use ibc_relayer_components::relay::traits::channel::open_try::ChannelOpenTryRelayerComponent;
use ibc_relayer_components::relay::traits::components::auto_relayer::AutoRelayerComponent;
use ibc_relayer_components::relay::traits::components::client_creator::ClientCreatorComponent;
use ibc_relayer_components::relay::traits::components::event_relayer::EventRelayerComponent;
use ibc_relayer_components::relay::traits::components::ibc_message_sender::{
    IbcMessageSenderComponent, MainSink,
};
use ibc_relayer_components::relay::traits::components::packet_clearer::PacketClearerComponent;
use ibc_relayer_components::relay::traits::components::packet_filter::PacketFilterComponent;
use ibc_relayer_components::relay::traits::components::packet_relayer::PacketRelayerComponent;
use ibc_relayer_components::relay::traits::components::packet_relayers::ack_packet::AckPacketRelayerComponent;
use ibc_relayer_components::relay::traits::components::packet_relayers::receive_packet::ReceivePacketRelayerComponnent;
use ibc_relayer_components::relay::traits::components::packet_relayers::timeout_unordered_packet::TimeoutUnorderedPacketRelayerComponent;
use ibc_relayer_components::relay::traits::components::update_client_message_builder::UpdateClientMessageBuilderComponent;
use ibc_relayer_components::relay::traits::connection::open_ack::ConnectionOpenAckRelayerComponent;
use ibc_relayer_components::relay::traits::connection::open_confirm::ConnectionOpenConfirmRelayerComponent;
use ibc_relayer_components::relay::traits::connection::open_handshake::ConnectionOpenHandshakeRelayerComponent;
use ibc_relayer_components::relay::traits::connection::open_init::ConnectionInitializerComponent;
use ibc_relayer_components::relay::traits::connection::open_try::ConnectionOpenTryRelayerComponent;

use crate::batch::components::message_sender::SendMessagesToBatchWorker;
use crate::batch::types::sink::BatchWorkerSink;
use crate::relay::components::packet_relayers::retry::RetryRelayer;

pub struct ExtraRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    ExtraRelayComponents<BaseComponents>;
    IbcMessageSenderComponent<MainSink>: SendMessagesToBatchWorker,
    IbcMessageSenderComponent<BatchWorkerSink>:
        SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>,
    PacketRelayerComponent:
        LockPacketRelayer<LoggerRelayer<FilterRelayer<RetryRelayer<FullCycleRelayer>>>>,
    [
        UpdateClientMessageBuilderComponent,
        PacketFilterComponent,
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
        DefaultRelayComponents<BaseComponents>,
);
