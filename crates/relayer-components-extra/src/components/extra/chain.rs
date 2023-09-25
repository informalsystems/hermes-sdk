use core::marker::PhantomData;

use cgp_core::{delegate_component, delegate_components};
use ibc_relayer_components::chain::traits::components::chain_status_querier::ChainStatusQuerierComponent;
use ibc_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;
use ibc_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use ibc_relayer_components::chain::traits::components::packet_fields_reader::PacketFieldsReaderComponent;
use ibc_relayer_components::components::default::chain::DefaultChainComponents;

use crate::telemetry::components::consensus_state::ConsensusStateTelemetryQuerier;
use crate::telemetry::components::status::ChainStatusTelemetryQuerier;

pub struct ExtraChainComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_component!(
    ChainStatusQuerierComponent,
    ExtraChainComponents<BaseComponents>,
    ChainStatusTelemetryQuerier<BaseComponents>,
);

delegate_component!(
    ConsensusStateQuerierComponent,
    ExtraChainComponents<BaseComponents>,
    ConsensusStateTelemetryQuerier<BaseComponents>,
);

delegate_components!(
    [MessageSenderComponent, PacketFieldsReaderComponent,],
    ExtraChainComponents<BaseComponents>,
    DefaultChainComponents<BaseComponents>,
);
