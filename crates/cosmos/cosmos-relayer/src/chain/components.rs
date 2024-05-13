use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use futures::lock::Mutex;
use hermes_cosmos_chain_components::components::client::CosmosClientComponents;
use hermes_cosmos_chain_components::components::cosmos_to_cosmos::CosmosToCosmosComponents;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::components::transaction::CosmosTxComponents;
use hermes_cosmos_chain_components::traits::abci_query::AbciQuerierComponent;
use hermes_cosmos_chain_components::traits::gas_config::GasConfigGetter;
use hermes_cosmos_chain_components::traits::tx_extension_options::TxExtensionOptionsGetter;
use hermes_cosmos_chain_components::types::nonce_guard::NonceGuard;
use hermes_cosmos_test_components::chain::components::CosmmosChainTestComponents;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent,
};
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::chain::traits::commitment_prefix::{
    CommitmentPrefixTypeComponent, IbcCommitmentPrefixGetter,
};
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::packet::fields::PacketFieldsReaderComponent;
use hermes_relayer_components::chain::traits::packet::from_write_ack::PacketFromWriteAckBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::AckPacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::ChannelHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    ConnectionOpenAckPayloadBuilderComponent, ConnectionOpenConfirmPayloadBuilderComponent,
    ConnectionOpenInitPayloadBuilderComponent, ConnectionOpenTryPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::timeout_unordered_packet::TimeoutUnorderedPacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::queries::ack_packets::{
    AckPacketQuerierComponent, AckPacketsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::block::BlockQuerierComponent;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, AllRawClientStatesQuerierComponent,
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
use hermes_relayer_components::chain::traits::types::block::{
    BlockHashComponent, BlockTypeComponent,
};
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelHandshakePayloadTypeComponent, InitChannelOptionsTypeComponent,
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
    CreateClientEventComponent, CreateClientOptionsTypeComponent, CreateClientPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::{
    GenesisHeightGetterComponent, HeightFieldComponent, HeightIncrementerComponent,
    HeightTypeComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::{
    MessageSizeEstimatorComponent, MessageTypeComponent,
};
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::packets::ack::AckPacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::receive::ReceivePacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::timeout::TimeoutUnorderedPacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::proof::CommitmentProofTypeComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::chain::traits::types::update_client::UpdateClientPayloadTypeComponent;
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_relayer_components::transaction::impls::poll_tx_response::PollTimeoutGetterComponent;
use hermes_relayer_components::transaction::traits::default_signer::DefaultSignerGetter;
use hermes_relayer_components::transaction::traits::encode_tx::TxEncoderComponent;
use hermes_relayer_components::transaction::traits::estimate_tx_fee::TxFeeEstimatorComponent;
use hermes_relayer_components::transaction::traits::nonce::allocate_nonce::NonceAllocatorComponent;
use hermes_relayer_components::transaction::traits::nonce::nonce_guard::NonceGuardComponent;
use hermes_relayer_components::transaction::traits::nonce::nonce_mutex::ProvideMutexForNonceAllocation;
use hermes_relayer_components::transaction::traits::nonce::query_nonce::NonceQuerierComponent;
use hermes_relayer_components::transaction::traits::parse_events::TxResponseAsEventsParserComponent;
use hermes_relayer_components::transaction::traits::poll_tx_response::TxResponsePollerComponent;
use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerierComponent;
use hermes_relayer_components::transaction::traits::send_messages_with_signer::MessagesWithSignerSenderComponent;
use hermes_relayer_components::transaction::traits::send_messages_with_signer_and_nonce::MessagesWithSignerAndNonceSenderComponent;
use hermes_relayer_components::transaction::traits::simulation_fee::FeeForSimulationGetter;
use hermes_relayer_components::transaction::traits::submit_tx::TxSubmitterComponent;
use hermes_relayer_components::transaction::traits::types::fee::FeeTypeComponent;
use hermes_relayer_components::transaction::traits::types::nonce::NonceTypeComponent;
use hermes_relayer_components::transaction::traits::types::signer::SignerTypeComponent;
use hermes_relayer_components::transaction::traits::types::transaction::TransactionTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_response::TxResponseTypeComponent;
use hermes_relayer_components_extra::components::extra::chain::ExtraChainComponents;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::mutex::MutexGuardOf;
use hermes_runtime_components::traits::runtime::RuntimeTypeComponent;
use hermes_test_components::chain::traits::assert::eventual_amount::EventualAmountAsserterComponent;
use hermes_test_components::chain::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain::traits::chain_id::ChainIdFromStringBuilderComponent;
use hermes_test_components::chain::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilderComponent;
use hermes_test_components::chain::traits::proposal::types::proposal_id::ProposalIdTypeComponent;
use hermes_test_components::chain::traits::proposal::types::proposal_status::ProposalStatusTypeComponent;
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
use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer::chain::cosmos::types::gas::GasConfig;
use ibc_relayer::keyring::Secp256k1KeyPair;

use crate::contexts::chain::CosmosChain;
use crate::contexts::encoding::ProvideCosmosEncoding;
use crate::contexts::logger::ProvideCosmosLogger;
use crate::impls::error::HandleCosmosError;

pub struct CosmosChainComponents;

impl HasComponents for CosmosChainComponents {
    type Components = CosmosBaseChainComponents;
}

impl HasComponents for CosmosChain {
    type Components = CosmosChainComponents;
}

delegate_components! {
    CosmosChainComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideCosmosLogger,
        [
            EncodingTypeComponent,
            EncodingGetterComponent,
            DefaultEncodingGetterComponent,
        ]:
            ProvideCosmosEncoding,
        [
            HeightTypeComponent,
            HeightFieldComponent,
            HeightIncrementerComponent,
            GenesisHeightGetterComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            MessageSizeEstimatorComponent,
            EventTypeComponent,
            ClientStateTypeComponent,
            ClientStateFieldsGetterComponent,
            RawClientStateTypeComponent,
            RawConsensusStateTypeComponent,
            ConsensusStateTypeComponent,
            IbcChainTypesComponent,
            ConnectionEndQuerierComponent,
            ConnectionEndWithProofsQuerierComponent,
            ConnectionEndTypeComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeComponent,
            BlockTypeComponent,
            BlockHashComponent,
            CommitmentPrefixTypeComponent,
            CommitmentProofTypeComponent,

            CreateClientEventComponent,

            CreateClientPayloadTypeComponent,
            UpdateClientPayloadTypeComponent,

            ConnectionOpenInitPayloadTypeComponent,
            ConnectionOpenTryPayloadTypeComponent,
            ConnectionOpenAckPayloadTypeComponent,
            ConnectionOpenConfirmPayloadTypeComponent,

            ChannelHandshakePayloadTypeComponent,
            ReceivePacketPayloadTypeComponent,
            AckPacketPayloadTypeComponent,
            TimeoutUnorderedPacketPayloadTypeComponent,

            PacketFieldsReaderComponent,
            WriteAckQuerierComponent,

            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
            RawClientStateQuerierComponent,
            RawClientStateWithProofsQuerierComponent,
            AllClientStatesQuerierComponent,
            AllRawClientStatesQuerierComponent,

            RawConsensusStateQuerierComponent,
            RawConsensusStateWithProofsQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,

            ConsensusStateHeightQuerierComponent,
            ConsensusStateHeightsQuerierComponent,

            CreateClientOptionsTypeComponent,
            CreateClientMessageBuilderComponent,
            CreateClientPayloadBuilderComponent,
            UpdateClientMessageBuilderComponent,
            UpdateClientPayloadBuilderComponent,
            CounterpartyChainIdQuerierComponent,

            ConnectionOpenInitMessageBuilderComponent,
            ConnectionOpenTryMessageBuilderComponent,
            ConnectionOpenAckMessageBuilderComponent,
            ConnectionOpenConfirmMessageBuilderComponent,
            ConnectionOpenInitPayloadBuilderComponent,
            ConnectionOpenTryPayloadBuilderComponent,
            ConnectionOpenAckPayloadBuilderComponent,
            ConnectionOpenConfirmPayloadBuilderComponent,

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
        [
            SignerTypeComponent,
            NonceTypeComponent,
            NonceGuardComponent,
            TransactionTypeComponent,
            TransactionHashTypeComponent,
            FeeTypeComponent,
            TxResponseTypeComponent,
            MessageSenderComponent,
            MessagesWithSignerSenderComponent,
            MessagesWithSignerAndNonceSenderComponent,
            NonceAllocatorComponent,
            TxResponsePollerComponent,
            PollTimeoutGetterComponent,
            TxResponseAsEventsParserComponent,
            TxResponseQuerierComponent,
            TxEncoderComponent,
            TxFeeEstimatorComponent,
            TxSubmitterComponent,
            NonceQuerierComponent,
        ]:
            CosmosTxComponents,
        [
            WalletTypeComponent,
            WalletSignerComponent,
            ChainIdFromStringBuilderComponent,
            AmountTypeComponent,
            AmountMethodsComponent,
            DenomTypeComponent,
            AddressTypeComponent,
            MemoTypeComponent,
            ProposalIdTypeComponent,
            ProposalStatusTypeComponent,
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
        [
            ChainStatusQuerierComponent,
            ConsensusStateQuerierComponent,
        ]:
            CosmosClientComponents,
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        CosmosChain: CosmosToCosmosComponents,
    }
}

impl TxExtensionOptionsGetter<CosmosChain> for CosmosChainComponents {
    fn tx_extension_options(chain: &CosmosChain) -> &Vec<ibc_proto::google::protobuf::Any> {
        &chain.tx_config.extension_options
    }
}

impl GasConfigGetter<CosmosChain> for CosmosChainComponents {
    fn gas_config(chain: &CosmosChain) -> &GasConfig {
        &chain.tx_config.gas_config
    }
}

impl DefaultSignerGetter<CosmosChain> for CosmosChainComponents {
    fn get_default_signer(chain: &CosmosChain) -> &Secp256k1KeyPair {
        &chain.key_entry
    }
}

impl FeeForSimulationGetter<CosmosChain> for CosmosChainComponents {
    fn fee_for_simulation(chain: &CosmosChain) -> &Fee {
        &chain.tx_config.gas_config.max_fee
    }
}

impl ProvideMutexForNonceAllocation<CosmosChain> for CosmosChainComponents {
    fn mutex_for_nonce_allocation<'a>(
        chain: &'a CosmosChain,
        _signer: &Secp256k1KeyPair,
    ) -> &'a Mutex<()> {
        &chain.nonce_mutex
    }

    fn mutex_to_nonce_guard<'a>(
        mutex_guard: MutexGuardOf<'a, HermesRuntime, ()>,
        account: Account,
    ) -> NonceGuard<'a> {
        NonceGuard {
            mutex_guard,
            account,
        }
    }
}

impl IbcCommitmentPrefixGetter<CosmosChain> for CosmosChainComponents {
    fn ibc_commitment_prefix(chain: &CosmosChain) -> &Vec<u8> {
        &chain.ibc_commitment_prefix
    }
}
