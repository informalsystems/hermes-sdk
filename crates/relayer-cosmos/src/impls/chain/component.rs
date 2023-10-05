use cgp_core::{delegate_component, Async, HasComponents};
use ibc_relayer_components::chain::traits::components::chain_status_querier::ChainStatusQuerierComponent;
use ibc_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerierComponent;
use ibc_relayer_components::chain::traits::components::connection_handshake_payload_builder::ConnectionHandshakePayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;
use ibc_relayer_components::chain::traits::components::counterparty_chain_id_querier::CounterpartyChainIdQuerierComponent;
use ibc_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use ibc_relayer_components::chain::traits::components::packet_fields_reader::PacketFieldsReaderComponent;
use ibc_relayer_components::chain::traits::components::write_ack_querier::WriteAckQuerierComponent;
use ibc_relayer_components_extra::components::extra::chain::ExtraChainComponents;

use crate::contexts::chain::CosmosChain;
use crate::impls::chain::components::connection_handshake_payload::BuildCosmosConnectionHandshakePayload;
use crate::impls::chain::components::create_client_message::BuildCosmosCreateClientMessage;
use crate::impls::chain::components::create_client_payload::BuildCreateClientPayloadWithChainHandle;
use crate::impls::chain::components::packet_fields::CosmosPacketFieldReader;
use crate::impls::chain::components::query_chain_id::QueryChainIdWithChainHandle;
use crate::impls::chain::components::query_chain_status::QueryChainStatusWithChainHandle;
use crate::impls::chain::components::query_client_state::QueryCosmosClientStateFromChainHandle;
use crate::impls::chain::components::query_consensus_state::QueryCosmosConsensusStateFromChainHandle;
use crate::impls::chain::components::query_write_ack_event::QueryWriteAckEventFromChainHandle;
use crate::impls::chain::components::send_messages_as_tx::SendMessagesToTxContext;

pub struct CosmosChainComponents;

impl<Chain> HasComponents for CosmosChain<Chain>
where
    Chain: Async,
{
    type Components = ExtraChainComponents<CosmosChainComponents>;
}

delegate_component!(
    MessageSenderComponent,
    CosmosChainComponents,
    SendMessagesToTxContext,
);

delegate_component!(
    ChainStatusQuerierComponent,
    CosmosChainComponents,
    QueryChainStatusWithChainHandle,
);

delegate_component!(
    PacketFieldsReaderComponent,
    CosmosChainComponents,
    CosmosPacketFieldReader,
);

delegate_component!(
    ClientStateQuerierComponent,
    CosmosChainComponents,
    QueryCosmosClientStateFromChainHandle,
);

delegate_component!(
    ConsensusStateQuerierComponent,
    CosmosChainComponents,
    QueryCosmosConsensusStateFromChainHandle,
);

delegate_component!(
    WriteAckQuerierComponent,
    CosmosChainComponents,
    QueryWriteAckEventFromChainHandle,
);

delegate_component!(
    CreateClientMessageBuilderComponent,
    CosmosChainComponents,
    BuildCosmosCreateClientMessage,
);

delegate_component!(
    CreateClientPayloadBuilderComponent,
    CosmosChainComponents,
    BuildCreateClientPayloadWithChainHandle,
);

delegate_component!(
    CounterpartyChainIdQuerierComponent,
    CosmosChainComponents,
    QueryChainIdWithChainHandle,
);

delegate_component!(
    ConnectionHandshakePayloadBuilderComponent,
    CosmosChainComponents,
    BuildCosmosConnectionHandshakePayload,
);
