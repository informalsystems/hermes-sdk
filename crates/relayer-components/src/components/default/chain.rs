use core::marker::PhantomData;

use cgp_core::delegate_components;

use crate::chain::traits::components::chain_status_querier::ChainStatusQuerierComponent;
use crate::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;
use crate::chain::traits::components::counterparty_chain_id_querier::CounterpartyChainIdQuerierComponent;
use crate::chain::traits::components::message_sender::MessageSenderComponent;
use crate::chain::traits::components::packet_commitments_querier::PacketCommitmentsQuerierComponent;
use crate::chain::traits::components::packet_fields_reader::PacketFieldsReaderComponent;
use crate::chain::traits::components::received_packet_querier::ReceivedPacketQuerierComponent;
use crate::chain::traits::components::send_packets_querier::SendPacketsQuerierComponent;
use crate::chain::traits::components::unreceived_packet_sequences_querier::UnreceivedPacketSequencesQuerierComponent;
use crate::chain::traits::components::write_ack_querier::WriteAckQuerierComponent;
pub struct DefaultChainComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    [
        ChainStatusQuerierComponent,
        ConsensusStateQuerierComponent,
        MessageSenderComponent,
        PacketFieldsReaderComponent,
        CounterpartyChainIdQuerierComponent,
        PacketCommitmentsQuerierComponent,
        ReceivedPacketQuerierComponent,
        SendPacketsQuerierComponent,
        UnreceivedPacketSequencesQuerierComponent,
        WriteAckQuerierComponent,
    ],
    DefaultChainComponents<BaseComponents>,
    BaseComponents,
);
