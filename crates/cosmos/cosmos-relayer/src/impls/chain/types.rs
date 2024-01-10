use hermes_cosmos_client_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_client_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_client_components::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};
use hermes_cosmos_client_components::types::payloads::client::{
    CosmosCreateClientPayload, CosmosUpdateClientPayload,
};
use hermes_cosmos_client_components::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};
use hermes_cosmos_client_components::types::payloads::packet::{
    CosmosAckPacketPayload, CosmosReceivePacketPayload, CosmosTimeoutUnorderedPacketPayload,
};
use hermes_cosmos_client_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloads, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionHandshakePayloads, HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientOptionsType, HasCreateClientPayload,
};
use hermes_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayload;
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayload;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayload;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer::chain::client::ClientSettings;

use crate::contexts::chain::CosmosChain;
use crate::impls::chain::component::CosmosChainComponents;
use crate::types::telemetry::CosmosTelemetry;

impl ProvideRuntime<CosmosChain> for CosmosChainComponents {
    fn runtime(chain: &CosmosChain) -> &HermesRuntime {
        &chain.runtime
    }
}

impl HasTelemetry for CosmosChain {
    type Telemetry = CosmosTelemetry;

    fn telemetry(&self) -> &CosmosTelemetry {
        &self.telemetry
    }
}

impl<Counterparty> HasClientStateType<Counterparty> for CosmosChain {
    type ClientState = TendermintClientState;
}

impl<Counterparty> HasConsensusStateType<Counterparty> for CosmosChain {
    type ConsensusState = TendermintConsensusState;
}

impl<Counterparty> HasCreateClientOptionsType<Counterparty> for CosmosChain {
    type CreateClientOptions = ClientSettings;
}

impl<Counterparty> HasInitConnectionOptionsType<Counterparty> for CosmosChain {
    type InitConnectionOptions = CosmosInitConnectionOptions;
}

impl<Counterparty> HasInitChannelOptionsType<Counterparty> for CosmosChain {
    type InitChannelOptions = CosmosInitChannelOptions;
}

impl<Counterparty> HasCreateClientPayload<Counterparty> for CosmosChain {
    type CreateClientPayload = CosmosCreateClientPayload;
}

impl<Counterparty> HasUpdateClientPayload<Counterparty> for CosmosChain {
    type UpdateClientPayload = CosmosUpdateClientPayload;
}

impl<Counterparty> HasConnectionHandshakePayloads<Counterparty> for CosmosChain {
    type ConnectionOpenInitPayload = CosmosConnectionOpenInitPayload;

    type ConnectionOpenTryPayload = CosmosConnectionOpenTryPayload;

    type ConnectionOpenAckPayload = CosmosConnectionOpenAckPayload;

    type ConnectionOpenConfirmPayload = CosmosConnectionOpenConfirmPayload;
}

impl<Counterparty> HasChannelHandshakePayloads<Counterparty> for CosmosChain {
    type ChannelOpenTryPayload = CosmosChannelOpenTryPayload;

    type ChannelOpenAckPayload = CosmosChannelOpenAckPayload;

    type ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload;
}

impl<Counterparty> HasReceivePacketPayload<Counterparty> for CosmosChain {
    type ReceivePacketPayload = CosmosReceivePacketPayload;
}

impl<Counterparty> HasAckPacketPayload<Counterparty> for CosmosChain {
    type AckPacketPayload = CosmosAckPacketPayload;
}

impl<Counterparty> HasTimeoutUnorderedPacketPayload<Counterparty> for CosmosChain {
    type TimeoutUnorderedPacketPayload = CosmosTimeoutUnorderedPacketPayload;
}
