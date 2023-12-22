use cgp_core::HasComponents;
use hermes_relayer_components::chain::traits::components::chain_status_querier::{
    CanQueryChainStatus, ChainStatusQuerier,
};
use hermes_relayer_components::chain::traits::components::client_state_querier::{
    CanQueryClientState, ClientStateQuerier,
};
use hermes_relayer_components::chain::traits::components::consensus_state_height_querier::{
    CanQueryConsensusStateHeight, ConsensusStateHeightQuerier,
};
use hermes_relayer_components::chain::traits::components::consensus_state_querier::{
    CanQueryConsensusState, ConsensusStateQuerier,
};
use hermes_relayer_components::chain::traits::components::message_sender::{
    CanSendMessages, MessageSender,
};
use hermes_relayer_components::chain::traits::components::update_client_message_builder::{
    CanBuildUpdateClientMessage, UpdateClientMessageBuilder,
};
use hermes_relayer_components::chain::traits::components::update_client_payload_builder::{
    CanBuildUpdateClientPayload, UpdateClientPayloadBuilder,
};
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::CanIncrementHeight;
use hermes_relayer_components::chain::traits::types::ibc::{
    HasCounterpartyMessageHeight, HasIbcChainTypes,
};
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;
use hermes_relayer_components::logger::traits::has_logger::HasLoggerType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;

use crate::components::extra::chain::DelegatesToExtraChainComponents;
use crate::telemetry::traits::metrics::HasBasicMetrics;
use crate::telemetry::traits::telemetry::HasTelemetry;

pub trait UseExtraChainComponentsForIbcMessageSender<Counterparty>:
    HasRuntime
    + HasChainId
    + HasLoggerType
    + CanIncrementHeight
    + CanSendMessages
    + CanQueryChainStatus
    + HasConsensusStateType<Counterparty>
    + HasClientStateFields<Counterparty>
    + HasCounterpartyMessageHeight<Counterparty>
    + CanQueryClientState<Counterparty>
    + CanQueryConsensusState<Counterparty>
    + CanQueryConsensusStateHeight<Counterparty>
    + CanBuildUpdateClientPayload<Counterparty>
    + CanBuildUpdateClientMessage<Counterparty>
where
    Counterparty: HasClientStateType<Self>
        + HasConsensusStateType<Self>
        + HasIbcChainTypes<Self>
        + HasUpdateClientPayload<Self>,
{
}

impl<Chain, Counterparty, Components, BaseComponents>
    UseExtraChainComponentsForIbcMessageSender<Counterparty> for Chain
where
    Chain: HasRuntime
        + HasChainId
        + HasLoggerType
        + CanIncrementHeight
        + HasTelemetry
        + HasChainStatusType
        + HasConsensusStateType<Counterparty>
        + HasClientStateFields<Counterparty>
        + HasCounterpartyMessageHeight<Counterparty>
        + HasIbcChainTypes<Counterparty>
        + HasClientStateType<Counterparty>
        + HasUpdateClientPayload<Counterparty>
        + HasComponents<Components = Components>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasConsensusStateType<Chain>
        + HasUpdateClientPayload<Chain>,
    Chain::Telemetry: HasBasicMetrics,
    Components: HasComponents<Components = BaseComponents>
        + DelegatesToExtraChainComponents<BaseComponents>
        + MessageSender<Chain>
        + ChainStatusQuerier<Chain>
        + ConsensusStateQuerier<Chain, Counterparty>
        + ClientStateQuerier<Chain, Counterparty>
        + ConsensusStateHeightQuerier<Chain, Counterparty>
        + UpdateClientPayloadBuilder<Chain, Counterparty>
        + UpdateClientMessageBuilder<Chain, Counterparty>,
{
}
