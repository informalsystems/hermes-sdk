use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::types::Error;
use hermes_relayer_components::relay::traits::chains::ProvideRelayChains;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::context::chain::MockSolomachine;
use crate::context::relay::SolomachineRelay;
use crate::impls::relay::component::SolomachineRelayComponents;

impl ProvideRelayChains<SolomachineRelay> for SolomachineRelayComponents {
    type SrcChain = MockSolomachine;

    type DstChain = CosmosChain;

    type Packet = Packet;

    fn src_client_id(relay: &SolomachineRelay) -> &ClientId {
        &relay.src_client_id
    }

    fn dst_client_id(relay: &SolomachineRelay) -> &ClientId {
        &relay.dst_client_id
    }

    fn src_chain(relay: &SolomachineRelay) -> &MockSolomachine {
        &relay.src_chain
    }

    fn dst_chain(relay: &SolomachineRelay) -> &CosmosChain {
        &relay.dst_chain
    }
}
