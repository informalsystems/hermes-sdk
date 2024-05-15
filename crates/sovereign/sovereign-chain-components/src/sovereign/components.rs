use cgp_core::prelude::*;
use hermes_relayer_components::chain::impls::forward::message_builders::channel_handshake::ForwardChannelHandshakeBuilder;
use hermes_relayer_components::chain::impls::forward::queries::chain_status::ForwardQueryChainStatus;
use hermes_relayer_components::chain::impls::forward::queries::client_state::ForwardQueryClientState;
use hermes_relayer_components::chain::impls::forward::queries::consensus_state::ForwardQueryConsensusState;
use hermes_relayer_components::chain::impls::forward::queries::consensus_state_height::ForwardQueryConsensusStateHeight;
use hermes_relayer_components::chain::traits::commitment_prefix::CommitmentPrefixTypeComponent;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::queries::connection_end::{
    ConnectionEndQuerierComponent, ConnectionEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::ChannelHandshakePayloadBuilderComponent;
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
    ChannelHandshakePayloadTypeComponent, InitChannelOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetterComponent, ClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionEndTypeComponent, ConnectionOpenAckPayloadTypeComponent, ConnectionOpenConfirmPayloadTypeComponent, ConnectionOpenInitPayloadTypeComponent, ConnectionOpenTryPayloadTypeComponent, InitConnectionOptionsTypeComponent
};
use hermes_relayer_components::chain::traits::types::consensus_state::ConsensusStateTypeComponent;
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientEventComponent, CreateClientOptionsTypeComponent, CreateClientPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::{
    HeightFieldComponent, HeightTypeComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::ibc_events::connection::ConnectionOpenInitEventComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
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
use hermes_sovereign_rollup_components::impls::cosmos_to_sovereign::client::create_client_message::BuildCreateCosmosClientMessageOnSovereign;
use hermes_sovereign_rollup_components::impls::cosmos_to_sovereign::client::update_client_message::BuildUpdateCosmosClientMessageOnSovereign;
use hermes_sovereign_rollup_components::impls::events::ProvideSovereignEvents;
use hermes_sovereign_rollup_components::impls::types::transaction::ProvideSovereignTransactionTypes;
use hermes_sovereign_rollup_components::impls::types::client_state::ProvideSovereignClientState;
use hermes_sovereign_rollup_components::impls::types::consensus_state::ProvideSovereignConsensusState;
use hermes_sovereign_rollup_components::impls::cosmos_to_sovereign::connection::connection_handshake_message::BuildCosmosConnectionHandshakeMessageOnSovereign;
use hermes_relayer_components::chain::impls::forward::queries::connection_end::ForwardQueryConnectionEnd;

use crate::sovereign::impls::sovereign_to_cosmos::channel::channel_handshake_payload::BuildSovereignChannelHandshakePayload;
use crate::sovereign::impls::sovereign_to_cosmos::client::create_client_payload::BuildSovereignCreateClientPayload;
use crate::sovereign::impls::sovereign_to_cosmos::client::update_client_payload::BuildSovereignUpdateClientPayload;
use crate::sovereign::impls::sovereign_to_cosmos::connection::connection_handshake_payload::BuildSovereignConnectionHandshakePayload;
use crate::sovereign::impls::types::chain::ProvideSovereignChainTypes;
use crate::sovereign::impls::types::payload::ProvideSovereignPayloadTypes;

pub struct SovereignChainClientComponents;

delegate_components! {
    #[mark_component(IsSovereignChainClientComponent)]
    SovereignChainClientComponents {
        [
            HeightTypeComponent,
            HeightFieldComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
            ChainStatusTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            CommitmentPrefixTypeComponent,
            CommitmentProofTypeComponent,
            ConnectionEndTypeComponent,
        ]:
            ProvideSovereignChainTypes,
        [
            CreateClientEventComponent,
            ConnectionOpenInitEventComponent,
        ]:
            ProvideSovereignEvents,
        [
            CreateClientOptionsTypeComponent,
            CreateClientPayloadTypeComponent,
            UpdateClientPayloadTypeComponent,
            InitConnectionOptionsTypeComponent,

            ConnectionOpenInitPayloadTypeComponent,
            ConnectionOpenTryPayloadTypeComponent,
            ConnectionOpenAckPayloadTypeComponent,
            ConnectionOpenConfirmPayloadTypeComponent,

            InitChannelOptionsTypeComponent,
            ChannelHandshakePayloadTypeComponent,
        ]:
            ProvideSovereignPayloadTypes,
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
            BuildSovereignConnectionHandshakePayload,

        [
            ConnectionOpenInitMessageBuilderComponent,
            ConnectionOpenTryMessageBuilderComponent,
            ConnectionOpenAckMessageBuilderComponent,
            ConnectionOpenConfirmMessageBuilderComponent,
        ]:
            BuildCosmosConnectionHandshakeMessageOnSovereign,

        ChannelHandshakePayloadBuilderComponent:
            BuildSovereignChannelHandshakePayload,
        ChannelHandshakeMessageBuilderComponent:
            ForwardChannelHandshakeBuilder,

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
        ConsensusStateHeightQuerierComponent:
            ForwardQueryConsensusStateHeight,
    }
}
