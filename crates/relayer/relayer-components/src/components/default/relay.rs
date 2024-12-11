pub use cgp::extra::run::RunnerComponent;
use cgp::prelude::*;

use crate::relay::impls::auto_relayers::both_targets::RelayBothTargets;
use crate::relay::impls::auto_relayers::event::RelayEvents;
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
use crate::relay::impls::packet_clearers::packets::ClearAllPackets;
use crate::relay::impls::packet_filters::chain::FilterRelayPacketWithChains;
use crate::relay::impls::packet_relayers::ack::base_ack_packet::BaseAckPacketRelayer;
use crate::relay::impls::packet_relayers::general::filter_relayer::FilterRelayer;
use crate::relay::impls::packet_relayers::general::full_relay::FullCycleRelayer;
use crate::relay::impls::packet_relayers::general::lock::LockPacketRelayer;
use crate::relay::impls::packet_relayers::general::log::LoggerRelayer;
use crate::relay::impls::packet_relayers::receive::base_receive_packet::BaseReceivePacketRelayer;
use crate::relay::impls::packet_relayers::receive::skip_received_packet::SkipReceivedPacketRelayer;
use crate::relay::impls::packet_relayers::timeout_unordered::timeout_unordered_packet::BaseTimeoutUnorderedPacketRelayer;
use crate::relay::impls::update_client::build::BuildUpdateClientMessages;
use crate::relay::impls::update_client::skip::SkipUpdateClient;
use crate::relay::impls::update_client::wait::WaitUpdateClient;
pub use crate::relay::traits::auto_relayer::AutoRelayerComponent;
pub use crate::relay::traits::channel::open_ack::ChannelOpenAckRelayerComponent;
pub use crate::relay::traits::channel::open_confirm::ChannelOpenConfirmRelayerComponent;
pub use crate::relay::traits::channel::open_handshake::ChannelOpenHandshakeRelayerComponent;
pub use crate::relay::traits::channel::open_init::ChannelInitializerComponent;
pub use crate::relay::traits::channel::open_try::ChannelOpenTryRelayerComponent;
pub use crate::relay::traits::client_creator::ClientCreatorComponent;
pub use crate::relay::traits::connection::open_ack::ConnectionOpenAckRelayerComponent;
pub use crate::relay::traits::connection::open_confirm::ConnectionOpenConfirmRelayerComponent;
pub use crate::relay::traits::connection::open_handshake::ConnectionOpenHandshakeRelayerComponent;
pub use crate::relay::traits::connection::open_init::ConnectionInitializerComponent;
pub use crate::relay::traits::connection::open_try::ConnectionOpenTryRelayerComponent;
pub use crate::relay::traits::event_relayer::EventRelayerComponent;
pub use crate::relay::traits::ibc_message_sender::{IbcMessageSenderComponent, MainSink};
pub use crate::relay::traits::packet_clearer::PacketClearerComponent;
pub use crate::relay::traits::packet_filter::RelayPacketFilterComponent;
pub use crate::relay::traits::packet_relayer::PacketRelayerComponent;
pub use crate::relay::traits::packet_relayers::ack_packet::AckPacketRelayerComponent;
pub use crate::relay::traits::packet_relayers::receive_packet::ReceivePacketRelayerComponent;
pub use crate::relay::traits::packet_relayers::timeout_unordered_packet::TimeoutUnorderedPacketRelayerComponent;
pub use crate::relay::traits::update_client_message_builder::TargetUpdateClientMessageBuilderComponent;

cgp_preset! {
    DefaultRelayComponents {
        IbcMessageSenderComponent<MainSink>: SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>,
        TargetUpdateClientMessageBuilderComponent: SkipUpdateClient<WaitUpdateClient<BuildUpdateClientMessages>>,
        PacketRelayerComponent: LockPacketRelayer<LoggerRelayer<FilterRelayer<FullCycleRelayer>>>,
        ReceivePacketRelayerComponent: SkipReceivedPacketRelayer<BaseReceivePacketRelayer>,
        AckPacketRelayerComponent: BaseAckPacketRelayer,
        TimeoutUnorderedPacketRelayerComponent: BaseTimeoutUnorderedPacketRelayer,
        EventRelayerComponent: PacketEventRelayer,
        RunnerComponent: RelayBothTargets,
        AutoRelayerComponent: RelayEvents,
        ClientCreatorComponent: CreateClientWithChains,
        PacketClearerComponent: ClearAllPackets,
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
    }
}
