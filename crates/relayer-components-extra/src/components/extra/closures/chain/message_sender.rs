use cgp_core::HasComponents;
use ibc_relayer_components::chain::traits::components::chain_status_querier::{
    CanQueryChainStatus, ChainStatusQuerier,
};
use ibc_relayer_components::chain::traits::components::client_state_querier::{
    CanQueryClientState, ClientStateQuerier,
};
use ibc_relayer_components::chain::traits::components::consensus_state_height_querier::{
    CanQueryConsensusStateHeight, ConsensusStateHeightQuerier,
};
use ibc_relayer_components::chain::traits::components::consensus_state_querier::{
    CanQueryConsensusState, ConsensusStateQuerier,
};
use ibc_relayer_components::chain::traits::components::message_sender::{
    CanSendMessages, MessageSender,
};
use ibc_relayer_components::chain::traits::components::update_client_message_builder::{
    CanBuildUpdateClientMessage, UpdateClientMessageBuilder,
};
use ibc_relayer_components::chain::traits::components::update_client_payload_builder::{
    CanBuildUpdateClientPayload, UpdateClientPayloadBuilder,
};
use ibc_relayer_components::chain::traits::logs::packet::CanLogChainPacket;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use ibc_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use ibc_relayer_components::chain::traits::types::height::CanIncrementHeight;
use ibc_relayer_components::chain::traits::types::ibc::{
    HasCounterpartyMessageHeight, HasIbcChainTypes,
};
use ibc_relayer_components::chain::traits::types::status::HasChainStatusType;
use ibc_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;
use ibc_relayer_components::logger::traits::has_logger::HasLoggerType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;

use crate::components::extra::chain::ExtraChainComponents;
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

impl<Chain, Counterparty, ChainComponents> UseExtraChainComponentsForIbcMessageSender<Counterparty>
    for Chain
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
        + CanLogChainPacket<Counterparty>
        + HasIbcChainTypes<Counterparty>
        + HasClientStateType<Counterparty>
        + HasUpdateClientPayload<Counterparty>
        + HasComponents<Components = ExtraChainComponents<ChainComponents>>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasConsensusStateType<Chain>
        + HasUpdateClientPayload<Chain>,
    Chain::Telemetry: HasBasicMetrics,
    ChainComponents: MessageSender<Chain>
        + ChainStatusQuerier<Chain>
        + ConsensusStateQuerier<Chain, Counterparty>
        + ClientStateQuerier<Chain, Counterparty>
        + ConsensusStateHeightQuerier<Chain, Counterparty>
        + UpdateClientPayloadBuilder<Chain, Counterparty>
        + UpdateClientMessageBuilder<Chain, Counterparty>,
{
}
