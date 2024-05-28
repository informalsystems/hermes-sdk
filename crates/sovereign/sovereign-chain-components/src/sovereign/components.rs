use cgp_core::prelude::*;
use hermes_relayer_components::chain::impls::types::payloads::channel::ProvideChannelPayloadTypes;
use hermes_relayer_components::chain::impls::types::payloads::connection::ProvideConnectionPayloadTypes;
use hermes_relayer_components::chain::impls::forward::queries::chain_status::ForwardQueryChainStatus;
use hermes_relayer_components::chain::impls::forward::queries::client_state::ForwardQueryClientState;
use hermes_relayer_components::chain::impls::forward::queries::consensus_state::ForwardQueryConsensusState;
use hermes_relayer_components::chain::impls::forward::queries::consensus_state_height::ForwardQueryConsensusStateHeight;
use hermes_relayer_components::chain::traits::commitment_prefix::CommitmentPrefixTypeComponent;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{ChannelOpenAckMessageBuilderComponent, ChannelOpenConfirmMessageBuilderComponent, ChannelOpenInitMessageBuilderComponent, ChannelOpenTryMessageBuilderComponent};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::queries::connection_end::{
    ConnectionEndQuerierComponent, ConnectionEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::channel_end::{
    ChannelEndQuerierComponent, ChannelEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::impls::forward::queries::channel_end::ForwardQueryChannelEnd;
use hermes_relayer_components::chain::traits::commitment_prefix::IbcCommitmentPrefixGetterComponent;
use hermes_cosmos_chain_components::impls::commitment_prefix::ProvideIbcCommitmentPrefix;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    ConnectionOpenAckPayloadBuilderComponent, ConnectionOpenConfirmPayloadBuilderComponent,
    ConnectionOpenInitPayloadBuilderComponent, ConnectionOpenTryPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::client_state::{ClientStateQuerierComponent, ClientStateWithProofsQuerierComponent};
use hermes_relayer_components::chain::traits::queries::consensus_state::{ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightQuerierComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelEndTypeComponent, ChannelOpenAckPayloadTypeComponent, ChannelOpenConfirmPayloadTypeComponent, ChannelOpenTryPayloadTypeComponent, InitChannelOptionsTypeComponent
};
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetterComponent, ClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionEndTypeComponent, ConnectionOpenAckPayloadTypeComponent, ConnectionOpenConfirmPayloadTypeComponent, ConnectionOpenInitPayloadTypeComponent, ConnectionOpenTryPayloadTypeComponent, InitConnectionOptionsTypeComponent
};
use hermes_cosmos_chain_components::impls::channel::channel_handshake_message::BuildCosmosChannelHandshakeMessage;
use hermes_relayer_components::chain::traits::queries::packet_receipt::PacketReceiptQuerierComponent;
use hermes_relayer_components::chain::traits::types::consensus_state::ConsensusStateTypeComponent;
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientEventComponent, CreateClientOptionsTypeComponent, CreateClientPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::{
    HeightFieldComponent, HeightIncrementerComponent, HeightTypeComponent
};
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilderComponent;
use hermes_cosmos_chain_components::impls::packet::packet_message::BuildCosmosPacketMessages;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::PacketAcknowledgementQuerierComponent;
use hermes_relayer_components::chain::impls::forward::queries::packet_receipt::ForwardQueryPacketReceipt;
use hermes_relayer_components::chain::impls::forward::queries::packet_acknowledgement::ForwardQueryPacketAcknowledgement;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::ibc_events::channel::{ChannelOpenInitEventComponent, ChannelOpenTryEventComponent};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::{ConnectionOpenInitEventComponent, ConnectionOpenTryEventComponent};
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::packets::ack::AcknowledgementTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::timeout::PacketReceiptTypeComponent;
use hermes_relayer_components::chain::traits::types::proof::CommitmentProofTypeComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::chain::traits::types::update_client::UpdateClientPayloadTypeComponent;
use hermes_relayer_components::transaction::traits::types::fee::FeeTypeComponent;
use hermes_relayer_components::transaction::traits::types::nonce::NonceTypeComponent;
use hermes_relayer_components::transaction::traits::types::signer::SignerTypeComponent;
use hermes_relayer_components::transaction::traits::types::transaction::TransactionTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_response::TxResponseTypeComponent;
use hermes_sovereign_rollup_components::impls::cosmos_to_sovereign::channel::channel_handshake_message::BuildChannelOpenInitMessageOnSovereign;
use hermes_sovereign_rollup_components::impls::cosmos_to_sovereign::client::create_client_message::BuildCreateCosmosClientMessageOnSovereign;
use hermes_sovereign_rollup_components::impls::cosmos_to_sovereign::client::update_client_message::BuildUpdateCosmosClientMessageOnSovereign;
use hermes_sovereign_rollup_components::impls::events::ProvideSovereignEvents;
use hermes_sovereign_rollup_components::impls::types::transaction::ProvideSovereignTransactionTypes;
use hermes_sovereign_rollup_components::impls::types::client_state::ProvideSovereignClientState;
use hermes_sovereign_rollup_components::impls::types::consensus_state::ProvideSovereignConsensusState;
use hermes_sovereign_rollup_components::impls::cosmos_to_sovereign::connection::connection_handshake_message::BuildCosmosConnectionHandshakeMessageOnSovereign;
use hermes_relayer_components::chain::impls::forward::queries::connection_end::ForwardQueryConnectionEnd;
use hermes_relayer_components::chain::impls::payload_builders::connection::BuildConnectionHandshakePayload;
use hermes_relayer_components::chain::traits::types::ibc::CounterpartyMessageHeightGetterComponent;
use hermes_sovereign_rollup_components::impls::message_height::GetCosmosHeightFromSovereignMessage;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    ChannelOpenTryPayloadBuilderComponent,
    ChannelOpenAckPayloadBuilderComponent,
    ChannelOpenConfirmPayloadBuilderComponent,
};
use hermes_relayer_components::chain::impls::payload_builders::channel::BuildChannelHandshakePayload;

use crate::sovereign::impls::sovereign_to_cosmos::client::create_client_payload::BuildSovereignCreateClientPayload;
use crate::sovereign::impls::sovereign_to_cosmos::client::update_client_payload::BuildSovereignUpdateClientPayload;
use crate::sovereign::impls::types::chain::ProvideSovereignChainTypes;
use crate::sovereign::impls::types::payload::ProvideSovereignPayloadTypes;

pub struct SovereignChainClientComponents;

delegate_components! {
    #[mark_component(IsSovereignChainClientComponent)]
    SovereignChainClientComponents {
        [
            HeightTypeComponent,
            HeightFieldComponent,
            HeightIncrementerComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
            ChainStatusTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            CommitmentPrefixTypeComponent,
            CommitmentProofTypeComponent,
            AcknowledgementTypeComponent,
            PacketReceiptTypeComponent,
            ConnectionEndTypeComponent,
            ChannelEndTypeComponent,
        ]:
            ProvideSovereignChainTypes,
        [
            CreateClientEventComponent,
            ConnectionOpenInitEventComponent,
            ConnectionOpenTryEventComponent,
            ChannelOpenInitEventComponent,
            ChannelOpenTryEventComponent,
        ]:
            ProvideSovereignEvents,
        [
            CreateClientOptionsTypeComponent,
            CreateClientPayloadTypeComponent,
            UpdateClientPayloadTypeComponent,
            InitConnectionOptionsTypeComponent,
            InitChannelOptionsTypeComponent,
        ]:
            ProvideSovereignPayloadTypes,
        [
            ConnectionOpenInitPayloadTypeComponent,
            ConnectionOpenTryPayloadTypeComponent,
            ConnectionOpenAckPayloadTypeComponent,
            ConnectionOpenConfirmPayloadTypeComponent,
        ]:
            ProvideConnectionPayloadTypes,
        [
            ChannelOpenTryPayloadTypeComponent,
            ChannelOpenAckPayloadTypeComponent,
            ChannelOpenConfirmPayloadTypeComponent,
        ]:
            ProvideChannelPayloadTypes,
        [
            ClientStateTypeComponent,
            ClientStateFieldsGetterComponent,
        ]:
            ProvideSovereignClientState,
        ConsensusStateTypeComponent:
            ProvideSovereignConsensusState,
        [
            TransactionTypeComponent,
            NonceTypeComponent,
            FeeTypeComponent,
            SignerTypeComponent,
            TransactionHashTypeComponent,
            TxResponseTypeComponent,
        ]:
            ProvideSovereignTransactionTypes,
        IbcCommitmentPrefixGetterComponent:
            ProvideIbcCommitmentPrefix,
        CreateClientPayloadBuilderComponent:
            BuildSovereignCreateClientPayload,
        CreateClientMessageBuilderComponent:
            BuildCreateCosmosClientMessageOnSovereign,
        UpdateClientPayloadBuilderComponent:
            BuildSovereignUpdateClientPayload,
        UpdateClientMessageBuilderComponent:
            BuildUpdateCosmosClientMessageOnSovereign,

        [
            ConnectionOpenInitPayloadBuilderComponent,
            ConnectionOpenTryPayloadBuilderComponent,
            ConnectionOpenAckPayloadBuilderComponent,
            ConnectionOpenConfirmPayloadBuilderComponent,
        ]:
            BuildConnectionHandshakePayload,
        [
            ConnectionOpenInitMessageBuilderComponent,
            ConnectionOpenTryMessageBuilderComponent,
            ConnectionOpenAckMessageBuilderComponent,
            ConnectionOpenConfirmMessageBuilderComponent,
        ]:
            BuildCosmosConnectionHandshakeMessageOnSovereign,

        [
            ChannelOpenTryPayloadBuilderComponent,
            ChannelOpenAckPayloadBuilderComponent,
            ChannelOpenConfirmPayloadBuilderComponent,
        ]:
            BuildChannelHandshakePayload,

        ChannelOpenInitMessageBuilderComponent:
            BuildChannelOpenInitMessageOnSovereign,

        [
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

        ChainStatusQuerierComponent:
            ForwardQueryChainStatus,
        [
            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
        ]:
            ForwardQueryClientState,
        [
            ConsensusStateQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,
        ]:
            ForwardQueryConsensusState,
        [
            ConnectionEndQuerierComponent,
            ConnectionEndWithProofsQuerierComponent,
        ]:
            ForwardQueryConnectionEnd,
        [
            ChannelEndQuerierComponent,
            ChannelEndWithProofsQuerierComponent,
        ]:
            ForwardQueryChannelEnd,
        PacketAcknowledgementQuerierComponent:
            ForwardQueryPacketAcknowledgement,
        PacketReceiptQuerierComponent:
            ForwardQueryPacketReceipt,
        ConsensusStateHeightQuerierComponent:
            ForwardQueryConsensusStateHeight,
        CounterpartyMessageHeightGetterComponent:
            GetCosmosHeightFromSovereignMessage,
    }
}
