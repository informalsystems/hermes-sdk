use hermes_relayer_components::relay::traits::chains::ProvideRelayChains;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::impls::relay::component::CosmosRelayComponents;

impl ProvideRelayChains<CosmosRelay> for CosmosRelayComponents {
    type SrcChain = CosmosChain;

    type DstChain = CosmosChain;

    type Packet = Packet;

    fn src_chain(relay: &CosmosRelay) -> &CosmosChain {
        &relay.src_chain
    }

    fn dst_chain(relay: &CosmosRelay) -> &CosmosChain {
        &relay.dst_chain
    }

    fn src_client_id(relay: &CosmosRelay) -> &ClientId {
        &relay.src_client_id
    }

    fn dst_client_id(relay: &CosmosRelay) -> &ClientId {
        &relay.dst_client_id
    }
}

impl ProvideRuntime<CosmosRelay> for CosmosRelayComponents {
    fn runtime(relay: &CosmosRelay) -> &HermesRuntime {
        &relay.runtime
    }
}
