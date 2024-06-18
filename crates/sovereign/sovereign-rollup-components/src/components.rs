use cgp_core::prelude::*;
use hermes_cosmos_chain_components::impls::channel::channel_handshake_message::BuildCosmosChannelHandshakeMessage;
use hermes_cosmos_chain_components::impls::connection::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;
use hermes_cosmos_chain_components::impls::packet::packet_message::BuildCosmosPacketMessages;
use hermes_cosmos_chain_components::impls::transaction::poll_timeout::DefaultPollTimeout;
use hermes_cosmos_chain_components::impls::types::client_state::ProvideAnyRawClientState;
use hermes_cosmos_chain_components::impls::types::consensus_state::ProvideAnyRawConsensusState;
use hermes_relayer_components::chain::impls::queries::consensus_state_height::QueryConsensusStateHeightsAndFindHeightBefore;
use hermes_relayer_components::chain::impls::queries::query_and_convert_client_state::QueryAndConvertRawClientState;
use hermes_relayer_components::chain::impls::queries::query_and_convert_consensus_state::QueryAndConvertRawConsensusState;
use hermes_relayer_components::chain::traits::commitment_prefix::{
    CommitmentPrefixTypeComponent, IbcCommitmentPrefixGetterComponent,
};
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilderComponent, ChannelOpenConfirmMessageBuilderComponent,
    ChannelOpenInitMessageBuilderComponent, ChannelOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::channel_end::{
    ChannelEndQuerierComponent, ChannelEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::client_state::{
    ClientStateQuerierComponent, ClientStateWithProofsQuerierComponent,
    RawClientStateQuerierComponent, RawClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::connection_end::{
    ConnectionEndQuerierComponent, ConnectionEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
    RawConsensusStateQuerierComponent, RawConsensusStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::{
    ConsensusStateHeightQuerierComponent, ConsensusStateHeightsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::PacketAcknowledgementQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_commitment::PacketCommitmentQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_receipt::PacketReceiptQuerierComponent;
use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelEndTypeComponent, ChannelOpenAckPayloadTypeComponent,
    ChannelOpenConfirmPayloadTypeComponent, ChannelOpenTryPayloadTypeComponent,
    InitChannelOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetterComponent, ClientStateTypeComponent, RawClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionEndTypeComponent, ConnectionOpenAckPayloadTypeComponent,
    ConnectionOpenConfirmPayloadTypeComponent, ConnectionOpenInitPayloadTypeComponent,
    ConnectionOpenTryPayloadTypeComponent, InitConnectionOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusStateTypeComponent, RawConsensusStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientEventComponent, CreateClientMessageOptionsTypeComponent,
    CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::{
    HeightFieldComponent, HeightIncrementerComponent, HeightTypeComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::{
    CounterpartyMessageHeightGetterComponent, IbcChainTypesComponent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::channel::{
    ChannelOpenInitEventComponent, ChannelOpenTryEventComponent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::{
    ConnectionOpenInitEventComponent, ConnectionOpenTryEventComponent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::SendPacketEventComponent;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::WriteAckEventComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::packets::ack::AcknowledgementTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::receive::PacketCommitmentTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::timeout::PacketReceiptTypeComponent;
use hermes_relayer_components::chain::traits::types::proof::CommitmentProofTypeComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::chain::traits::types::update_client::UpdateClientPayloadTypeComponent;
use hermes_relayer_components::components::default::transaction::DefaultTxComponents;
use hermes_relayer_components::transaction::impls::poll_tx_response::PollTimeoutGetterComponent;
use hermes_relayer_components::transaction::traits::encode_tx::TxEncoderComponent;
use hermes_relayer_components::transaction::traits::estimate_tx_fee::TxFeeEstimatorComponent;
use hermes_relayer_components::transaction::traits::nonce::allocate_nonce::NonceAllocatorComponent;
use hermes_relayer_components::transaction::traits::nonce::nonce_guard::NonceGuardComponent;
use hermes_relayer_components::transaction::traits::nonce::query_nonce::NonceQuerierComponent;
use hermes_relayer_components::transaction::traits::parse_events::TxResponseAsEventsParserComponent;
use hermes_relayer_components::transaction::traits::poll_tx_response::TxResponsePollerComponent;
use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerierComponent;
use hermes_relayer_components::transaction::traits::send_messages_with_signer::MessagesWithSignerSenderComponent;
use hermes_relayer_components::transaction::traits::send_messages_with_signer_and_nonce::MessagesWithSignerAndNonceSenderComponent;
use hermes_relayer_components::transaction::traits::simulation_fee::FeeForSimulationGetterComponent;
use hermes_relayer_components::transaction::traits::submit_tx::TxSubmitterComponent;
use hermes_relayer_components::transaction::traits::types::fee::FeeTypeComponent;
use hermes_relayer_components::transaction::traits::types::nonce::NonceTypeComponent;
use hermes_relayer_components::transaction::traits::types::signer::SignerTypeComponent;
use hermes_relayer_components::transaction::traits::types::transaction::TransactionTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_response::TxResponseTypeComponent;

use crate::impls::commitment_prefix::ProvideSovereignIbcCommitmentPrefix;
use crate::impls::cosmos_to_sovereign::client::create_client_message::BuildCreateCosmosClientMessageOnSovereign;
use crate::impls::cosmos_to_sovereign::client::update_client_message::BuildUpdateCosmosClientMessageOnSovereign;
use crate::impls::events::ProvideSovereignEvents;
use crate::impls::json_rpc_client::ProvideJsonRpseeClient;
use crate::impls::message_height::GetCosmosHeightFromSovereignMessage;
use crate::impls::queries::chain_status::QuerySovereignRollupStatus;
use crate::impls::queries::channel_end::QueryChannelEndOnSovereign;
use crate::impls::queries::client_state::QueryClientStateOnSovereign;
use crate::impls::queries::connection_end::QueryConnectionEndOnSovereign;
use crate::impls::queries::consensus_state::QueryConsensusStateOnSovereign;
use crate::impls::queries::consensus_state_height::QueryConsensusStateHeightsOnSovereign;
use crate::impls::queries::packet_acknowledgement::QueryPacketAcknowledgementFromSovereign;
use crate::impls::queries::packet_commitment::QueryPacketCommitmentFromSovereign;
use crate::impls::queries::packet_receipt::QueryPacketReceiptFromSovereign;
use crate::impls::send_message::SendMessagesInSequence;
use crate::impls::transaction::encode_tx::EncodeSovereignTx;
use crate::impls::transaction::estimate_fee::ReturnSovereignTxFee;
use crate::impls::transaction::event::ParseSovTxResponseAsEvents;
use crate::impls::transaction::query_nonce::QuerySovereignNonce;
use crate::impls::transaction::query_tx_response::QuerySovereignTxResponse;
use crate::impls::transaction::submit_tx::SubmitSovereignTransaction;
use crate::impls::types::client_state::ProvideSovereignClientState;
use crate::impls::types::consensus_state::ProvideSovereignConsensusState;
use crate::impls::types::payload::ProvideSovereignRollupPayloadTypes;
use crate::impls::types::rollup::ProvideSovereignRollupTypes;
use crate::impls::types::transaction::ProvideSovereignTransactionTypes;
use crate::traits::json_rpc_client::JsonRpcClientTypeComponent;

pub struct SovereignRollupClientComponents;

delegate_components! {
    #[mark_component(IsSovereignRollupClientComponent)]
    SovereignRollupClientComponents {
        [
            HeightTypeComponent,
            HeightFieldComponent,
            HeightIncrementerComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeComponent,
            CommitmentPrefixTypeComponent,
            CommitmentProofTypeComponent,
            PacketCommitmentTypeComponent,
            AcknowledgementTypeComponent,
            PacketReceiptTypeComponent,
            ConnectionEndTypeComponent,
            ChannelEndTypeComponent,
        ]:
            ProvideSovereignRollupTypes,
        [
            ClientStateTypeComponent,
            ClientStateFieldsGetterComponent,
        ]:
            ProvideSovereignClientState,
        ConsensusStateTypeComponent:
            ProvideSovereignConsensusState,
        [
            CreateClientEventComponent,
            ConnectionOpenInitEventComponent,
            ConnectionOpenTryEventComponent,
            ChannelOpenInitEventComponent,
            ChannelOpenTryEventComponent,
            SendPacketEventComponent,
            WriteAckEventComponent,
        ]:
            ProvideSovereignEvents,
        [
            CreateClientPayloadOptionsTypeComponent,
            CreateClientMessageOptionsTypeComponent,
            CreateClientPayloadTypeComponent,
            UpdateClientPayloadTypeComponent,
            InitConnectionOptionsTypeComponent,

            ConnectionOpenInitPayloadTypeComponent,
            ConnectionOpenTryPayloadTypeComponent,
            ConnectionOpenAckPayloadTypeComponent,
            ConnectionOpenConfirmPayloadTypeComponent,

            InitChannelOptionsTypeComponent,
            ChannelOpenTryPayloadTypeComponent,
            ChannelOpenAckPayloadTypeComponent,
            ChannelOpenConfirmPayloadTypeComponent,
        ]:
            ProvideSovereignRollupPayloadTypes,
        [
            TransactionTypeComponent,
            NonceTypeComponent,
            FeeTypeComponent,
            SignerTypeComponent,
            TransactionHashTypeComponent,
            TxResponseTypeComponent,
            NonceGuardComponent,
        ]:
            ProvideSovereignTransactionTypes,
        [
            NonceAllocatorComponent,
            MessagesWithSignerSenderComponent,
            MessagesWithSignerAndNonceSenderComponent,
            TxResponsePollerComponent,
        ]:
            DefaultTxComponents,
        MessageSenderComponent:
            SendMessagesInSequence<DefaultTxComponents>,
        IbcCommitmentPrefixGetterComponent:
            ProvideSovereignIbcCommitmentPrefix,
        JsonRpcClientTypeComponent:
            ProvideJsonRpseeClient,
        TxResponseQuerierComponent:
            QuerySovereignTxResponse,
        PollTimeoutGetterComponent:
            DefaultPollTimeout,
        TxResponseAsEventsParserComponent:
            ParseSovTxResponseAsEvents,
        TxEncoderComponent:
            EncodeSovereignTx,
        [
            TxFeeEstimatorComponent,
            FeeForSimulationGetterComponent,
        ]:
            ReturnSovereignTxFee<10_000>,
        TxSubmitterComponent:
            SubmitSovereignTransaction,
        NonceQuerierComponent:
            QuerySovereignNonce,
        CreateClientMessageBuilderComponent:
            BuildCreateCosmosClientMessageOnSovereign,
        UpdateClientMessageBuilderComponent:
            BuildUpdateCosmosClientMessageOnSovereign,
        ChainStatusQuerierComponent:
            QuerySovereignRollupStatus,
        RawClientStateTypeComponent:
            ProvideAnyRawClientState,
        [
            RawClientStateQuerierComponent,
            RawClientStateWithProofsQuerierComponent,
        ]:
            QueryClientStateOnSovereign,
        [
            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
        ]:
            QueryAndConvertRawClientState,
        RawConsensusStateTypeComponent:
            ProvideAnyRawConsensusState,
        [
            RawConsensusStateQuerierComponent,
            RawConsensusStateWithProofsQuerierComponent,
        ]:
            QueryConsensusStateOnSovereign,
        [
            ConsensusStateQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,
        ]:
            QueryAndConvertRawConsensusState,
        [
            ConnectionEndQuerierComponent,
            ConnectionEndWithProofsQuerierComponent,
        ]:
            QueryConnectionEndOnSovereign,
        [
            ChannelEndQuerierComponent,
            ChannelEndWithProofsQuerierComponent,
        ]:
            QueryChannelEndOnSovereign,
        PacketCommitmentQuerierComponent:
            QueryPacketCommitmentFromSovereign,
        PacketAcknowledgementQuerierComponent:
            QueryPacketAcknowledgementFromSovereign,
        PacketReceiptQuerierComponent:
            QueryPacketReceiptFromSovereign,
        ConsensusStateHeightsQuerierComponent:
            QueryConsensusStateHeightsOnSovereign,
        ConsensusStateHeightQuerierComponent:
            QueryConsensusStateHeightsAndFindHeightBefore,

        [
            ConnectionOpenInitMessageBuilderComponent,
            ConnectionOpenTryMessageBuilderComponent,
            ConnectionOpenAckMessageBuilderComponent,
            ConnectionOpenConfirmMessageBuilderComponent,
        ]:
            BuildCosmosConnectionHandshakeMessage,
        [
            ChannelOpenInitMessageBuilderComponent,
            ChannelOpenTryMessageBuilderComponent,
            ChannelOpenAckMessageBuilderComponent,
            ChannelOpenConfirmMessageBuilderComponent,
        ]:
            BuildCosmosChannelHandshakeMessage,

        [
            ReceivePacketMessageBuilderComponent,
            AckPacketMessageBuilderComponent,
            TimeoutUnorderedPacketMessageBuilderComponent,
        ]:
            BuildCosmosPacketMessages,

        CounterpartyMessageHeightGetterComponent:
            GetCosmosHeightFromSovereignMessage,
    }
}
