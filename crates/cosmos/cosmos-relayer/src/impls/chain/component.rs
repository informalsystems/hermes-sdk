use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use hermes_cosmos_client_components::components::all::CosmosClientComponents;
use hermes_relayer_components::chain::traits::components::ack_packet_message_builder::AckPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::ack_packet_payload_builder::AckPacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::chain_status_querier::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::components::channel_handshake_message_builder::ChannelHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::channel_handshake_payload_builder::ChannelHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerierComponent;
use hermes_relayer_components::chain::traits::components::connection_handshake_message_builder::ConnectionHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::connection_handshake_payload_builder::ConnectionHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::consensus_state_height_querier::ConsensusStateHeightQuerierComponent;
use hermes_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;
use hermes_relayer_components::chain::traits::components::counterparty_chain_id_querier::CounterpartyChainIdQuerierComponent;
use hermes_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use hermes_relayer_components::chain::traits::components::packet_commitments_querier::PacketCommitmentsQuerierComponent;
use hermes_relayer_components::chain::traits::components::packet_fields_reader::PacketFieldsReaderComponent;
use hermes_relayer_components::chain::traits::components::packet_from_write_ack_builder::PacketFromWriteAckBuilderComponent;
use hermes_relayer_components::chain::traits::components::receive_packet_message_builder::ReceivePacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::receive_packet_payload_builder::ReceivePacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::received_packet_querier::ReceivedPacketQuerierComponent;
use hermes_relayer_components::chain::traits::components::send_packets_querier::{
    SendPacketQuerierComponent, SendPacketsQuerierComponent,
};
use hermes_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::{
    TimeoutUnorderedPacketMessageBuilderComponent, TimeoutUnorderedPacketPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::components::unreceived_packet_sequences_querier::UnreceivedPacketSequencesQuerierComponent;
use hermes_relayer_components::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::update_client_payload_builder::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::write_ack_querier::WriteAckQuerierComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::create_client::CreateClientOptionsTypeComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeProviderComponent;
use hermes_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_components_extra::components::extra::chain::ExtraChainComponents;
use hermes_relayer_components_extra::components::extra::chain::IsExtraChainComponent;
use hermes_relayer_components_extra::components::extra::closures::chain::all::CanUseExtraChainComponents;
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;

use crate::contexts::chain::CosmosChain;
use crate::impls::chain::components::connection_handshake_message::DelegateCosmosConnectionHandshakeBuilder;
use crate::impls::chain::components::create_client_message::DelegateCosmosCreateClientMessageBuilder;
use crate::impls::chain::components::query_client_state::DelegateCosmosClientStateQuerier;
use crate::impls::chain::components::query_consensus_state::DelegateCosmosConsensusStateQuerier;
use crate::impls::error::HandleCosmosError;

pub struct CosmosChainComponents;

pub struct CosmosBaseChainComponents;

impl HasComponents for CosmosChainComponents {
    type Components = CosmosBaseChainComponents;
}

impl HasComponents for CosmosChain {
    type Components = CosmosChainComponents;
}

delegate_all!(
    IsExtraChainComponent,
    ExtraChainComponents<CosmosBaseChainComponents>,
    CosmosChainComponents,
);

impl CanUseExtraChainComponents<CosmosChain> for CosmosChain {}

delegate_components! {
    CosmosChainComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeComponent:
            ProvideTokioRuntimeType,
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
        [
            HeightTypeProviderComponent,
            TimestampTypeProviderComponent,
            ChainIdTypeProviderComponent,
            MessageTypeProviderComponent,
            EventTypeProviderComponent,
            IbcChainTypesProviderComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeProviderComponent,
            MessageSenderComponent,
            PacketFieldsReaderComponent,
            ConsensusStateHeightQuerierComponent,
            WriteAckQuerierComponent,
            CreateClientOptionsTypeComponent,
            CreateClientPayloadBuilderComponent,
            UpdateClientPayloadBuilderComponent,
            UpdateClientMessageBuilderComponent,
            CounterpartyChainIdQuerierComponent,
            ConnectionHandshakePayloadBuilderComponent,
            ChannelHandshakePayloadBuilderComponent,
            ChannelHandshakeMessageBuilderComponent,
            PacketCommitmentsQuerierComponent,
            ReceivedPacketQuerierComponent,
            ReceivePacketPayloadBuilderComponent,
            ReceivePacketMessageBuilderComponent,
            AckPacketPayloadBuilderComponent,
            AckPacketMessageBuilderComponent,
            TimeoutUnorderedPacketPayloadBuilderComponent,
            TimeoutUnorderedPacketMessageBuilderComponent,
            UnreceivedPacketSequencesQuerierComponent,
            SendPacketQuerierComponent,
            SendPacketsQuerierComponent,
            PacketFromWriteAckBuilderComponent,
        ]:
            CosmosClientComponents,
        ClientStateQuerierComponent:
            DelegateCosmosClientStateQuerier,
        CreateClientMessageBuilderComponent:
            DelegateCosmosCreateClientMessageBuilder,
        ConnectionHandshakeMessageBuilderComponent:
            DelegateCosmosConnectionHandshakeBuilder,
    }
}

delegate_components! {
    CosmosBaseChainComponents {
        ChainStatusQuerierComponent:
            CosmosClientComponents,
        ConsensusStateQuerierComponent:
            DelegateCosmosConsensusStateQuerier,
    }
}
