use core::marker::PhantomData;

use cgp_core::delegate_components;

use crate::chain::traits::components::chain_status_querier::ChainStatusQuerierComponent;
use crate::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;
use crate::chain::traits::components::counterparty_chain_id_querier::CounterpartyChainIdQuerierComponent;
use crate::chain::traits::components::message_sender::MessageSenderComponent;
use crate::chain::traits::components::packet_commitments_querier::PacketCommitmentsQuerierComponent;
use crate::chain::traits::components::packet_fields_reader::PacketFieldsReaderComponent;
use crate::chain::traits::components::received_packet_querier::ReceivedPacketQuerierComponent;
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
    ],
    DefaultChainComponents<BaseComponents>,
    BaseComponents,
);
