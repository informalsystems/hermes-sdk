use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use hermes_cosmos_client_components::components::client::CosmosClientComponents;
use hermes_cosmos_client_components::traits::abci_query::AbciQuerierComponent;
use hermes_cosmos_test_components::chain::components::CosmmosChainTestComponents;
use hermes_protobuf_components::traits::encoding::ProtobufEncodingGetterComponent;
use hermes_protobuf_components::traits::encoding::ProtobufEncodingTypeComponent;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::packet::fields::PacketFieldsReaderComponent;
use hermes_relayer_components::chain::traits::packet::from_write_ack::PacketFromWriteAckBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::AckPacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::ChannelHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::ConnectionHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::timeout_unordered_packet::TimeoutUnorderedPacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::queries::ack_packets::AckPacketQuerierComponent;
use hermes_relayer_components::chain::traits::queries::ack_packets::AckPacketsQuerierComponent;
use hermes_relayer_components::chain::traits::queries::block::BlockQuerierComponent;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesBytesQuerierComponent, AllClientStatesQuerierComponent,
    ClientStateBytesQuerierComponent, ClientStateQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::connection_end::ConnectionEndQuerierComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerierComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightQuerierComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightsQuerierComponent;
use hermes_relayer_components::chain::traits::queries::counterparty_chain_id::CounterpartyChainIdQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgements::PacketAcknowledgementsQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_commitments::PacketCommitmentsQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_is_received::ReceivedPacketQuerierComponent;
use hermes_relayer_components::chain::traits::queries::send_packets::{
    SendPacketQuerierComponent, SendPacketsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::unreceived_acks_sequences::UnreceivedAcksSequencesQuerierComponent;
use hermes_relayer_components::chain::traits::queries::unreceived_packet_sequences::UnreceivedPacketSequencesQuerierComponent;
use hermes_relayer_components::chain::traits::queries::write_ack::WriteAckQuerierComponent;
use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
use hermes_relayer_components::chain::traits::types::block::BlockHashComponent;
use hermes_relayer_components::chain::traits::types::block::BlockTypeComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::channel::ChannelHandshakePayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::channel::InitChannelOptionsTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoderComponent, ClientStateFieldsGetterComponent, ClientStateTypeComponent,
    ClientStatesDecoderComponent,
};
use hermes_relayer_components::chain::traits::types::connection::ConnectionEndTypeComponent;
use hermes_relayer_components::chain::traits::types::connection::ConnectionHandshakePayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::connection::InitConnectionOptionsTypeComponent;
use hermes_relayer_components::chain::traits::types::consensus_state::ConsensusStateTypeComponent;
use hermes_relayer_components::chain::traits::types::create_client::CreateClientOptionsTypeComponent;
use hermes_relayer_components::chain::traits::types::create_client::CreateClientPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::GenesisHeightGetterComponent;
use hermes_relayer_components::chain::traits::types::height::HeightIncrementerComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::packets::ack::AckPacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::receive::ReceivePacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::timeout::TimeoutUnorderedPacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::chain::traits::types::update_client::UpdateClientPayloadTypeComponent;
use hermes_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_components_extra::components::extra::chain::ExtraChainComponents;
use hermes_relayer_components_extra::components::extra::closures::chain::all::CanUseExtraChainComponents;
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_test_components::chain::traits::assert::eventual_amount::EventualAmountAsserterComponent;
use hermes_test_components::chain::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain::traits::chain_id::ChainIdFromStringBuilderComponent;
use hermes_test_components::chain::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilderComponent;
use hermes_test_components::chain::traits::queries::balance::BalanceQuerierComponent;
use hermes_test_components::chain::traits::transfer::amount::IbcTransferredAmountConverterComponent;
use hermes_test_components::chain::traits::transfer::ibc_transfer::TokenIbcTransferrerComponent;
use hermes_test_components::chain::traits::transfer::timeout::IbcTransferTimeoutCalculatorComponent;
use hermes_test_components::chain::traits::types::address::AddressTypeComponent;
use hermes_test_components::chain::traits::types::amount::{
    AmountMethodsComponent, AmountTypeComponent,
};
use hermes_test_components::chain::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain::traits::types::memo::{
    DefaultMemoGetterComponent, MemoTypeComponent,
};
use hermes_test_components::chain::traits::types::wallet::{
    WalletSignerComponent, WalletTypeComponent,
};

use crate::chain::impls::connection_handshake_message::DelegateCosmosConnectionHandshakeBuilder;
use crate::chain::impls::create_client_message::DelegateCosmosCreateClientMessageBuilder;
use crate::chain::impls::query_consensus_state::DelegateCosmosConsensusStateQuerier;
use crate::chain::impls::update_client_message::DelegateCosmosUpdateClientMessageBuilder;
use crate::contexts::chain::CosmosChain;
use crate::contexts::encoding::ProvideCosmosEncoding;
use crate::impls::error::HandleCosmosError;

pub struct CosmosChainComponents;

impl HasComponents for CosmosChainComponents {
    type Components = CosmosBaseChainComponents;
}

impl HasComponents for CosmosChain {
    type Components = CosmosChainComponents;
}

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
            ProtobufEncodingTypeComponent,
            ProtobufEncodingGetterComponent,
        ]:
            ProvideCosmosEncoding,
        [
            HeightTypeComponent,
            HeightIncrementerComponent,
            GenesisHeightGetterComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
            ClientStateTypeComponent,
            ClientStateFieldsGetterComponent,
            ClientStateDecoderComponent,
            ClientStatesDecoderComponent,
            ConsensusStateTypeComponent,
            IbcChainTypesComponent,
            ConnectionEndQuerierComponent,
            ConnectionEndTypeComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeComponent,
            BlockTypeComponent,
            BlockHashComponent,

            CreateClientPayloadTypeComponent,
            UpdateClientPayloadTypeComponent,
            ConnectionHandshakePayloadTypeComponent,
            ChannelHandshakePayloadTypeComponent,
            ReceivePacketPayloadTypeComponent,
            AckPacketPayloadTypeComponent,
            TimeoutUnorderedPacketPayloadTypeComponent,

            MessageSenderComponent,
            PacketFieldsReaderComponent,
            ConsensusStateHeightQuerierComponent,
            ConsensusStateHeightsQuerierComponent,
            WriteAckQuerierComponent,
            ClientStateQuerierComponent,
            ClientStateBytesQuerierComponent,
            AllClientStatesQuerierComponent,
            AllClientStatesBytesQuerierComponent,
            CreateClientOptionsTypeComponent,
            CreateClientPayloadBuilderComponent,
            UpdateClientPayloadBuilderComponent,
            CounterpartyChainIdQuerierComponent,
            ConnectionHandshakePayloadBuilderComponent,
            ChannelHandshakePayloadBuilderComponent,
            ChannelHandshakeMessageBuilderComponent,
            PacketCommitmentsQuerierComponent,
            PacketAcknowledgementsQuerierComponent,
            ReceivedPacketQuerierComponent,
            ReceivePacketPayloadBuilderComponent,
            ReceivePacketMessageBuilderComponent,
            AckPacketPayloadBuilderComponent,
            AckPacketMessageBuilderComponent,
            TimeoutUnorderedPacketPayloadBuilderComponent,
            TimeoutUnorderedPacketMessageBuilderComponent,
            UnreceivedPacketSequencesQuerierComponent,
            UnreceivedAcksSequencesQuerierComponent,
            AckPacketQuerierComponent,
            AckPacketsQuerierComponent,
            SendPacketQuerierComponent,
            SendPacketsQuerierComponent,
            PacketFromWriteAckBuilderComponent,
            InitConnectionOptionsTypeComponent,
            InitChannelOptionsTypeComponent,
            BlockQuerierComponent,
            AbciQuerierComponent,
        ]:
            CosmosClientComponents,
        [
            ChainStatusQuerierComponent,
            ConsensusStateQuerierComponent,
        ]:
            ExtraChainComponents<CosmosBaseChainComponents>,
        CreateClientMessageBuilderComponent:
            DelegateCosmosCreateClientMessageBuilder,
        UpdateClientMessageBuilderComponent:
            DelegateCosmosUpdateClientMessageBuilder,
        ConnectionHandshakeMessageBuilderComponent:
            DelegateCosmosConnectionHandshakeBuilder,
        [
            WalletTypeComponent,
            WalletSignerComponent,
            ChainIdFromStringBuilderComponent,
            AmountTypeComponent,
            AmountMethodsComponent,
            DenomTypeComponent,
            AddressTypeComponent,
            MemoTypeComponent,
            DefaultMemoGetterComponent,
            TokenIbcTransferrerComponent,
            IbcTransferTimeoutCalculatorComponent,
            IbcTokenTransferMessageBuilderComponent,
            IbcTransferredAmountConverterComponent,
            BalanceQuerierComponent,
            EventualAmountAsserterComponent,
            PollAssertDurationGetterComponent,
        ]:
            CosmmosChainTestComponents,
    }
}

pub struct CosmosBaseChainComponents;

delegate_components! {
    CosmosBaseChainComponents {
        ChainStatusQuerierComponent:
            CosmosClientComponents,
        ConsensusStateQuerierComponent:
            DelegateCosmosConsensusStateQuerier,
    }
}
