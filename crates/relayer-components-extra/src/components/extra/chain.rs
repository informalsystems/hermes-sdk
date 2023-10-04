use core::marker::PhantomData;

use cgp_core::{delegate_component, delegate_components};
use ibc_relayer_components::chain::traits::components::chain_status_querier::ChainStatusQuerierComponent;
use ibc_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;
use ibc_relayer_components::chain::traits::components::counterparty_chain_id_querier::CounterpartyChainIdQuerierComponent;
use ibc_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use ibc_relayer_components::chain::traits::components::packet_commitments_querier::PacketCommitmentsQuerierComponent;
use ibc_relayer_components::chain::traits::components::packet_fields_reader::PacketFieldsReaderComponent;
use ibc_relayer_components::chain::traits::components::received_packet_querier::ReceivedPacketQuerierComponent;
use ibc_relayer_components::chain::traits::components::send_packets_querier::SendPacketsQuerierComponent;
use ibc_relayer_components::chain::traits::components::unreceived_packet_sequences_querier::UnreceivedPacketSequencesQuerierComponent;
use ibc_relayer_components::chain::traits::components::write_ack_querier::WriteAckQuerierComponent;
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
    [
        MessageSenderComponent,
        PacketFieldsReaderComponent,
        CounterpartyChainIdQuerierComponent,
        PacketCommitmentsQuerierComponent,
        ReceivedPacketQuerierComponent,
        SendPacketsQuerierComponent,
        UnreceivedPacketSequencesQuerierComponent,
        WriteAckQuerierComponent,
    ],
    ExtraChainComponents<BaseComponents>,
    DefaultChainComponents<BaseComponents>,
);
