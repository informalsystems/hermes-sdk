use core::ops::Deref;
use core::time::Duration;
use std::borrow::Cow;
use std::sync::Arc;

use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use futures::lock::Mutex;
use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_cli_components::any_client::contexts::any_counterparty::AnyCounterparty;
use hermes_cosmos_chain_components::components::client::CosmosClientComponents;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::components::transaction::CosmosTxComponents;
use hermes_cosmos_chain_components::traits::abci_query::{AbciQuerierComponent, CanQueryAbci};
use hermes_cosmos_chain_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_chain_components::traits::gas_config::GasConfigGetter;
use hermes_cosmos_chain_components::traits::grpc_address::GrpcAddressGetter;
use hermes_cosmos_chain_components::traits::rpc_client::RpcClientGetter;
use hermes_cosmos_chain_components::traits::tx_extension_options::TxExtensionOptionsGetter;
use hermes_cosmos_chain_components::types::nonce_guard::NonceGuard;
use hermes_cosmos_chain_components::types::tendermint::TendermintConsensusState;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::logger::ProvideCosmosLogger;
use hermes_cosmos_relayer::impls::error::HandleCosmosError;
use hermes_cosmos_relayer::types::error::Error;
use hermes_cosmos_relayer::types::telemetry::CosmosTelemetry;
use hermes_cosmos_test_components::chain::components::CosmmosChainTestComponents;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent,
    HasDefaultEncoding,
};
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, HasLogger, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::chain::traits::commitment_prefix::{
    CommitmentPrefixTypeComponent, IbcCommitmentPrefixGetter,
};
use hermes_relayer_components::chain::traits::event_subscription::HasEventSubscription;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::{
    AckPacketMessageBuilderComponent, CanBuildAckPacketMessage,
};
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    CanBuildChannelOpenAckMessage, CanBuildChannelOpenConfirmMessage,
    CanBuildChannelOpenInitMessage, CanBuildChannelOpenTryMessage,
    ChannelOpenAckMessageBuilderComponent, ChannelOpenConfirmMessageBuilderComponent,
    ChannelOpenInitMessageBuilderComponent, ChannelOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    CanBuildConnectionOpenAckMessage, CanBuildConnectionOpenConfirmMessage,
    CanBuildConnectionOpenInitMessage, CanBuildConnectionOpenTryMessage,
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::{
    CanBuildCreateClientMessage, CreateClientMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::receive_packet::{
    CanBuildReceivePacketMessage, ReceivePacketMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::{
    CanBuildTimeoutUnorderedPacketMessage, TimeoutUnorderedPacketMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::update_client::{
    CanBuildUpdateClientMessage, UpdateClientMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::packet::fields::PacketFieldsReaderComponent;
use hermes_relayer_components::chain::traits::packet::from_write_ack::PacketFromWriteAckBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::{
    AckPacketPayloadBuilderComponent, CanBuildAckPacketPayload,
};
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    CanBuildChannelOpenAckPayload, CanBuildChannelOpenConfirmPayload,
    CanBuildChannelOpenTryPayload, ChannelOpenAckPayloadBuilderComponent,
    ChannelOpenConfirmPayloadBuilderComponent, ChannelOpenTryPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    CanBuildConnectionOpenAckPayload, CanBuildConnectionOpenConfirmPayload,
    CanBuildConnectionOpenInitPayload, CanBuildConnectionOpenTryPayload,
    ConnectionOpenAckPayloadBuilderComponent, ConnectionOpenConfirmPayloadBuilderComponent,
    ConnectionOpenInitPayloadBuilderComponent, ConnectionOpenTryPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::{
    CanBuildReceivePacketPayload, ReceivePacketPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::timeout_unordered_packet::{
    CanBuildTimeoutUnorderedPacketPayload, TimeoutUnorderedPacketPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::queries::ack_packets::{
    AckPacketQuerierComponent, AckPacketsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::block::BlockQuerierComponent;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::channel_end::{
    CanQueryChannelEnd, CanQueryChannelEndWithProofs, ChannelEndQuerierComponent,
    ChannelEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, AllRawClientStatesQuerierComponent, CanQueryAllClientStates,
    CanQueryClientState, CanQueryClientStateWithProofs, CanQueryRawClientState,
    ClientStateQuerierComponent, ClientStateWithProofsQuerierComponent,
    RawClientStateQuerierComponent, RawClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::connection_end::{
    CanQueryConnectionEnd, CanQueryConnectionEndWithProofs, ConnectionEndQuerierComponent,
    ConnectionEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    CanQueryConsensusState, CanQueryConsensusStateWithProofs, CanQueryRawConsensusState,
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
    RawConsensusStateQuerierComponent, RawConsensusStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::{
    ConsensusStateHeightQuerierComponent, ConsensusStateHeightsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::counterparty_chain_id::CounterpartyChainIdQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::{
    CanQueryPacketAcknowledgement, PacketAcknowledgementQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::packet_acknowledgements::PacketAcknowledgementsQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_commitment::{
    CanQueryPacketCommitment, PacketCommitmentQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::packet_commitments::PacketCommitmentsQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_is_received::ReceivedPacketQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_receipt::{
    CanQueryPacketReceipt, PacketReceiptQuerierComponent,
};
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
use hermes_relayer_components::chain::traits::types::chain_id::{
    ChainIdGetter, ChainIdTypeComponent,
};
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelEndTypeComponent, ChannelOpenAckPayloadTypeComponent,
    ChannelOpenConfirmPayloadTypeComponent, ChannelOpenTryPayloadTypeComponent, HasChannelEndType,
    InitChannelOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetter, HasClientStateType, HasRawClientStateType, ProvideClientStateType,
    RawClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionEndTypeComponent, ConnectionOpenAckPayloadTypeComponent,
    ConnectionOpenConfirmPayloadTypeComponent, ConnectionOpenInitPayloadTypeComponent,
    ConnectionOpenTryPayloadTypeComponent, HasInitConnectionOptionsType,
    InitConnectionOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusStateTypeComponent, HasConsensusStateFields, HasConsensusStateType,
    RawConsensusStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientEventComponent, CreateClientMessageOptionsTypeComponent,
    CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
    HasCreateClientMessageOptionsType,
};
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::{
    GenesisHeightGetterComponent, HasHeightType, HeightFieldComponent, HeightIncrementerComponent,
    HeightTypeComponent,
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
use hermes_relayer_components::chain::traits::types::message::{
    MessageSizeEstimatorComponent, MessageTypeComponent,
};
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::packets::ack::AckPacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::ack::AcknowledgementTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::receive::{
    PacketCommitmentTypeComponent, ReceivePacketPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::packets::timeout::{
    PacketReceiptTypeComponent, TimeoutUnorderedPacketPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::proof::CommitmentProofTypeComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::{
    HasTimestampType, TimestampTypeComponent,
};
use hermes_relayer_components::chain::traits::types::update_client::UpdateClientPayloadTypeComponent;
use hermes_relayer_components::error::traits::retry::{HasRetryableError, RetryableErrorComponent};
use hermes_relayer_components::transaction::impls::poll_tx_response::{
    HasPollTimeout, PollTimeoutGetterComponent,
};
use hermes_relayer_components::transaction::traits::default_signer::DefaultSignerGetter;
use hermes_relayer_components::transaction::traits::encode_tx::TxEncoderComponent;
use hermes_relayer_components::transaction::traits::estimate_tx_fee::TxFeeEstimatorComponent;
use hermes_relayer_components::transaction::traits::nonce::allocate_nonce::NonceAllocatorComponent;
use hermes_relayer_components::transaction::traits::nonce::nonce_guard::NonceGuardComponent;
use hermes_relayer_components::transaction::traits::nonce::nonce_mutex::ProvideMutexForNonceAllocation;
use hermes_relayer_components::transaction::traits::nonce::query_nonce::NonceQuerierComponent;
use hermes_relayer_components::transaction::traits::parse_events::TxResponseAsEventsParserComponent;
use hermes_relayer_components::transaction::traits::poll_tx_response::{
    CanPollTxResponse, TxResponsePollerComponent,
};
use hermes_relayer_components::transaction::traits::query_tx_response::{
    CanQueryTxResponse, TxResponseQuerierComponent,
};
use hermes_relayer_components::transaction::traits::send_messages_with_signer::MessagesWithSignerSenderComponent;
use hermes_relayer_components::transaction::traits::send_messages_with_signer_and_nonce::MessagesWithSignerAndNonceSenderComponent;
use hermes_relayer_components::transaction::traits::simulation_fee::FeeForSimulationGetter;
use hermes_relayer_components::transaction::traits::submit_tx::{
    CanSubmitTx, TxSubmitterComponent,
};
use hermes_relayer_components::transaction::traits::types::fee::FeeTypeComponent;
use hermes_relayer_components::transaction::traits::types::nonce::NonceTypeComponent;
use hermes_relayer_components::transaction::traits::types::signer::SignerTypeComponent;
use hermes_relayer_components::transaction::traits::types::transaction::TransactionTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_response::TxResponseTypeComponent;
use hermes_relayer_components_extra::components::extra::chain::ExtraChainComponents;
use hermes_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::mutex::MutexGuardOf;
use hermes_runtime_components::traits::runtime::{HasRuntime, RuntimeGetter, RuntimeTypeComponent};
use hermes_test_components::chain::traits::assert::eventual_amount::EventualAmountAsserterComponent;
use hermes_test_components::chain::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain::traits::chain_id::ChainIdFromStringBuilderComponent;
use hermes_test_components::chain::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilderComponent;
use hermes_test_components::chain::traits::proposal::types::proposal_id::ProposalIdTypeComponent;
use hermes_test_components::chain::traits::proposal::types::proposal_status::ProposalStatusTypeComponent;
use hermes_test_components::chain::traits::queries::balance::{
    BalanceQuerierComponent, CanQueryBalance,
};
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
use http::Uri;
use ibc::core::channel::types::channel::ChannelEnd;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer::chain::cosmos::types::gas::GasConfig;
use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer::keyring::Secp256k1KeyPair;
use ibc_relayer_types::core::ics02_client::client_state::ClientState;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::Height;
use prost_types::Any;
use tendermint::abci::Event as AbciEvent;
use tendermint_rpc::{HttpClient, Url};

use crate::components::cosmos_to_wasm_cosmos::CosmosToWasmCosmosComponents;
use crate::context::encoding::{ProvideWasmCosmosEncoding, WasmCosmosEncoding};
use crate::types::client_state::WrappedTendermintClientState;

#[derive(Clone)]
pub struct WasmCosmosChain {
    pub chain: CosmosChain,
}

pub struct WasmCosmosChainComponents;

impl Deref for WasmCosmosChain {
    type Target = CosmosChain;

    fn deref(&self) -> &CosmosChain {
        &self.chain
    }
}

impl HasComponents for WasmCosmosChain {
    type Components = WasmCosmosChainComponents;
}

delegate_components! {
    WasmCosmosChainComponents {
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
            ProvideWasmCosmosEncoding,
    }
}

delegate_components! {
    WasmCosmosChainComponents {
        [
            CreateClientMessageOptionsTypeComponent,
            CreateClientMessageBuilderComponent,
            CreateClientPayloadTypeComponent,
            CreateClientPayloadOptionsTypeComponent,
            CreateClientPayloadBuilderComponent,
            HeightTypeComponent,
            HeightFieldComponent,
            HeightIncrementerComponent,
            GenesisHeightGetterComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            MessageSizeEstimatorComponent,
            EventTypeComponent,
            RawClientStateTypeComponent,
            RawConsensusStateTypeComponent,
            ConsensusStateTypeComponent,
            IbcChainTypesComponent,

            ConnectionEndQuerierComponent,
            ConnectionEndWithProofsQuerierComponent,
            ConnectionEndTypeComponent,

            ChannelEndQuerierComponent,
            ChannelEndWithProofsQuerierComponent,
            ChannelEndTypeComponent,

            IbcPacketTypesProviderComponent,
            ChainStatusTypeComponent,
            BlockTypeComponent,
            BlockHashComponent,
            CommitmentPrefixTypeComponent,
            CommitmentProofTypeComponent,
            PacketCommitmentTypeComponent,
            AcknowledgementTypeComponent,
            PacketReceiptTypeComponent,

            CreateClientEventComponent,
            ConnectionOpenInitEventComponent,
            ConnectionOpenTryEventComponent,
            ChannelOpenInitEventComponent,
            ChannelOpenTryEventComponent,
            SendPacketEventComponent,
            WriteAckEventComponent,

            UpdateClientPayloadTypeComponent,

            ConnectionOpenInitPayloadTypeComponent,
            ConnectionOpenTryPayloadTypeComponent,
            ConnectionOpenAckPayloadTypeComponent,
            ConnectionOpenConfirmPayloadTypeComponent,

            ChannelOpenTryPayloadTypeComponent,
            ChannelOpenAckPayloadTypeComponent,
            ChannelOpenConfirmPayloadTypeComponent,

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

            ChannelOpenTryPayloadBuilderComponent,
            ChannelOpenAckPayloadBuilderComponent,
            ChannelOpenConfirmPayloadBuilderComponent,

            ChannelOpenInitMessageBuilderComponent,
            ChannelOpenTryMessageBuilderComponent,
            ChannelOpenAckMessageBuilderComponent,
            ChannelOpenConfirmMessageBuilderComponent,

            PacketCommitmentQuerierComponent,
            PacketCommitmentsQuerierComponent,

            PacketAcknowledgementQuerierComponent,
            PacketAcknowledgementsQuerierComponent,

            PacketReceiptQuerierComponent,

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
            CounterpartyMessageHeightGetterComponent,
        ]:
            CosmosClientComponents,
    }
}

delegate_components! {
    WasmCosmosChainComponents {
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
        WasmCosmosChain: CosmosToWasmCosmosComponents,
    }
}

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty> for WasmCosmosChainComponents
where
    Chain: Async,
{
    type ClientState = WrappedTendermintClientState;
}

impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty> for WasmCosmosChainComponents
where
    Chain: HasClientStateType<Counterparty, ClientState = WrappedTendermintClientState>
        + HasHeightType<Height = Height>,
{
    fn client_state_latest_height(client_state: &WrappedTendermintClientState) -> Height {
        client_state.tendermint_client_state.latest_height
    }

    fn client_state_is_frozen(client_state: &WrappedTendermintClientState) -> bool {
        client_state.tendermint_client_state.is_frozen()
    }

    fn client_state_has_expired(
        client_state: &WrappedTendermintClientState,
        elapsed: Duration,
    ) -> bool {
        elapsed > client_state.tendermint_client_state.trusting_period
    }
}

impl TxExtensionOptionsGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn tx_extension_options(chain: &WasmCosmosChain) -> &Vec<ibc_proto::google::protobuf::Any> {
        &chain.tx_config.extension_options
    }
}

impl GasConfigGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn gas_config(chain: &WasmCosmosChain) -> &GasConfig {
        &chain.tx_config.gas_config
    }
}

impl DefaultSignerGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn get_default_signer(chain: &WasmCosmosChain) -> &Secp256k1KeyPair {
        &chain.key_entry
    }
}

impl FeeForSimulationGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn fee_for_simulation(chain: &WasmCosmosChain) -> &Fee {
        &chain.tx_config.gas_config.max_fee
    }
}

impl ProvideMutexForNonceAllocation<WasmCosmosChain> for WasmCosmosChainComponents {
    fn mutex_for_nonce_allocation<'a>(
        chain: &'a WasmCosmosChain,
        _signer: &Secp256k1KeyPair,
    ) -> &'a Mutex<()> {
        &chain.nonce_mutex
    }

    fn mutex_to_nonce_guard(
        mutex_guard: MutexGuardOf<'_, HermesRuntime, ()>,
        account: Account,
    ) -> NonceGuard<'_> {
        NonceGuard {
            mutex_guard,
            account,
        }
    }
}

impl IbcCommitmentPrefixGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn ibc_commitment_prefix(chain: &WasmCosmosChain) -> &Vec<u8> {
        &chain.ibc_commitment_prefix
    }
}

impl GrpcAddressGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn grpc_address(chain: &WasmCosmosChain) -> &Uri {
        &chain.tx_config.grpc_address
    }
}

impl RpcClientGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn rpc_client(chain: &WasmCosmosChain) -> &HttpClient {
        &chain.rpc_client
    }

    fn rpc_address(chain: &WasmCosmosChain) -> &Url {
        &chain.tx_config.rpc_address
    }
}

impl HasBlockingChainHandle for WasmCosmosChain {
    type ChainHandle = BaseChainHandle;

    async fn with_blocking_chain_handle<R>(
        &self,
        cont: impl FnOnce(BaseChainHandle) -> Result<R, Error> + Send + 'static,
    ) -> Result<R, Error>
    where
        R: Send + 'static,
    {
        let chain_handle = self.handle.clone();

        self.runtime
            .runtime
            .spawn_blocking(move || cont(chain_handle))
            .await?
    }
}

impl RuntimeGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn runtime(chain: &WasmCosmosChain) -> &HermesRuntime {
        &chain.runtime
    }
}

impl HasTelemetry for WasmCosmosChain {
    type Telemetry = CosmosTelemetry;

    fn telemetry(&self) -> &CosmosTelemetry {
        &self.telemetry
    }
}

impl ChainIdGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn chain_id(chain: &WasmCosmosChain) -> &ChainId {
        &chain.chain_id
    }
}

impl HasEventSubscription for WasmCosmosChain {
    fn event_subscription(&self) -> &Arc<dyn Subscription<Item = (Height, Arc<AbciEvent>)>> {
        &self.subscription
    }
}

impl<Counterparty> HasConsensusStateFields<Counterparty> for WasmCosmosChain
where
    Counterparty: HasTimestampType,
{
    fn consensus_state_timestamp(
        consensus_state: &Self::ConsensusState,
    ) -> Cow<'_, Counterparty::Timestamp> {
        // FIXME(romac): This is a temporary workaround until we have a proper conversion,
        // and can blow out if the timestamp is later than July 21st, 2554.
        let nanos = consensus_state.timestamp.unix_timestamp_nanos() as u64;
        Cow::Owned(Counterparty::timestamp_from_nanos(nanos))
    }
}

pub trait CanUseWasmCosmosChain:
    HasClientStateType<WasmCosmosChain, ClientState = WrappedTendermintClientState>
    + HasConsensusStateType<WasmCosmosChain, ConsensusState = TendermintConsensusState>
    + CanQueryBalance
    // + CanIbcTransferToken<WasmCosmosChain>
    // + CanBuildIbcTokenTransferMessage<WasmCosmosChain>
    + CanQueryRawClientState<WasmCosmosChain>
    + CanQueryClientState<WasmCosmosChain>
    + CanQueryClientStateWithProofs<WasmCosmosChain>
    + CanQueryConsensusState<WasmCosmosChain>
    + CanQueryConsensusStateWithProofs<WasmCosmosChain>
    + CanQueryRawConsensusState<WasmCosmosChain>
    + CanQueryAllClientStates<WasmCosmosChain>
    + CanQueryClientState<AnyCounterparty>
    + CanQueryAllClientStates<AnyCounterparty>
    + CanBuildUpdateClientMessage<WasmCosmosChain>
    + CanQueryConnectionEnd<WasmCosmosChain>
    + CanQueryChannelEnd<WasmCosmosChain>
    + CanQueryChannelEndWithProofs<WasmCosmosChain>
    + CanQueryConnectionEndWithProofs<WasmCosmosChain>
    + CanQueryPacketCommitment<WasmCosmosChain>
    + CanQueryPacketAcknowledgement<WasmCosmosChain>
    + CanQueryPacketReceipt<WasmCosmosChain>
    + HasChannelEndType<WasmCosmosChain, ChannelEnd = ChannelEnd>
    + HasRawClientStateType<RawClientState = Any>
    + CanSubmitTx
    + CanPollTxResponse
    + HasPollTimeout
    + CanQueryTxResponse
    + HasRetryableError
    + HasLogger
    + HasRuntime
    // + CanAssertEventualAmount
    + CanQueryAbci
    + CanQueryClientState<CosmosChain>
    + CanQueryConsensusState<CosmosChain>
    + CanBuildConnectionOpenInitPayload<CosmosChain>
    + CanBuildConnectionOpenTryPayload<CosmosChain>
    + CanBuildConnectionOpenAckPayload<CosmosChain>
    + CanBuildConnectionOpenConfirmPayload<CosmosChain>
    + CanBuildConnectionOpenInitMessage<CosmosChain>
    + CanBuildConnectionOpenTryMessage<CosmosChain>
    + CanBuildConnectionOpenAckMessage<CosmosChain>
    + CanBuildConnectionOpenConfirmMessage<CosmosChain>
    + CanBuildChannelOpenTryPayload<CosmosChain>
    + CanBuildChannelOpenAckPayload<CosmosChain>
    + CanBuildChannelOpenConfirmPayload<CosmosChain>
    + CanBuildChannelOpenInitMessage<CosmosChain>
    + CanBuildChannelOpenTryMessage<CosmosChain>
    + CanBuildChannelOpenAckMessage<CosmosChain>
    + CanBuildChannelOpenConfirmMessage<CosmosChain>
    + CanBuildReceivePacketPayload<CosmosChain>
    + CanBuildAckPacketPayload<CosmosChain>
    + CanBuildTimeoutUnorderedPacketPayload<CosmosChain>
    + CanBuildReceivePacketMessage<CosmosChain>
    + CanBuildAckPacketMessage<CosmosChain>
    + CanBuildTimeoutUnorderedPacketMessage<CosmosChain>
    + HasInitConnectionOptionsType<CosmosChain>
    + HasCreateClientMessageOptionsType<
            CosmosChain,
            CreateClientMessageOptions = (),
        >
    + HasDefaultEncoding<Encoding = WasmCosmosEncoding>
{
}

impl CanUseWasmCosmosChain for WasmCosmosChain {}

pub trait CanUseCosmosChainWithWasmCosmosChain:
    CanQueryClientState<WasmCosmosChain>
    + CanQueryConsensusState<WasmCosmosChain>
    + CanBuildConnectionOpenInitPayload<WasmCosmosChain>
    + CanBuildConnectionOpenTryPayload<WasmCosmosChain>
    + CanBuildConnectionOpenAckPayload<WasmCosmosChain>
    + CanBuildConnectionOpenConfirmPayload<WasmCosmosChain>
    + CanBuildConnectionOpenInitMessage<WasmCosmosChain>
    + CanBuildConnectionOpenTryMessage<WasmCosmosChain>
    + CanBuildChannelOpenTryPayload<WasmCosmosChain>
    + CanBuildChannelOpenAckPayload<WasmCosmosChain>
    + CanBuildChannelOpenConfirmPayload<WasmCosmosChain>
    + CanBuildChannelOpenInitMessage<WasmCosmosChain>
    + CanBuildChannelOpenTryMessage<WasmCosmosChain>
    + CanBuildChannelOpenAckMessage<WasmCosmosChain>
    + CanBuildChannelOpenConfirmMessage<WasmCosmosChain>
    + CanBuildReceivePacketPayload<WasmCosmosChain>
    + CanBuildAckPacketPayload<WasmCosmosChain>
    + CanBuildTimeoutUnorderedPacketPayload<WasmCosmosChain>
    + CanBuildReceivePacketMessage<WasmCosmosChain>
    + CanBuildAckPacketMessage<WasmCosmosChain>
    + CanBuildTimeoutUnorderedPacketMessage<WasmCosmosChain>
    + HasInitConnectionOptionsType<WasmCosmosChain>
    + CanBuildCreateClientMessage<WasmCosmosChain>
{
}

impl CanUseCosmosChainWithWasmCosmosChain for CosmosChain {}
