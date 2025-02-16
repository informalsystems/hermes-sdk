use cgp::prelude::HasProvider;
use hermes_chain_type_components::traits::fields::height::CanIncrementHeight;
use hermes_relayer_components::chain::traits::message_builders::update_client::{
    CanBuildUpdateClientMessage, UpdateClientMessageBuilder,
};
use hermes_relayer_components::chain::traits::payload_builders::update_client::{
    CanBuildUpdateClientPayload, UpdateClientPayloadBuilder,
};
use hermes_relayer_components::chain::traits::queries::chain_status::{
    CanQueryChainStatus, ChainStatusQuerier,
};
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryClientState, ClientStateQuerier,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    CanQueryConsensusState, ConsensusStateQuerier,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::{
    CanQueryConsensusStateHeight, ConsensusStateHeightQuerier,
};
use hermes_relayer_components::chain::traits::send_message::{CanSendMessages, MessageSender};
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::ibc::{
    HasCounterpartyMessageHeight, HasIbcChainTypes,
};
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::telemetry::traits::metrics::HasBasicMetrics;
use crate::telemetry::traits::telemetry::HasTelemetry;

pub trait UseExtraChainComponentsForIbcMessageSender<Counterparty>:
    HasRuntime
    + HasChainId
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
        + HasUpdateClientPayloadType<Self>,
{
}

impl<Chain, Counterparty, Components> UseExtraChainComponentsForIbcMessageSender<Counterparty>
    for Chain
where
    Chain: HasRuntime
        + HasChainId
        + CanIncrementHeight
        + HasTelemetry
        + HasChainStatusType
        + HasConsensusStateType<Counterparty>
        + HasClientStateFields<Counterparty>
        + HasCounterpartyMessageHeight<Counterparty>
        + HasIbcChainTypes<Counterparty>
        + HasClientStateType<Counterparty>
        + HasUpdateClientPayloadType<Counterparty>
        + HasProvider<Components = Components>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasConsensusStateType<Chain>
        + HasUpdateClientPayloadType<Chain>,
    Chain::Telemetry: HasBasicMetrics,
    Components: MessageSender<Chain>
        + ChainStatusQuerier<Chain>
        + ConsensusStateQuerier<Chain, Counterparty>
        + ClientStateQuerier<Chain, Counterparty>
        + ConsensusStateHeightQuerier<Chain, Counterparty>
        + UpdateClientPayloadBuilder<Chain, Counterparty>
        + UpdateClientMessageBuilder<Chain, Counterparty>,
{
}
