use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_runtime::types::runtime::HermesRuntime;

use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::impls::relay::component::CosmosRelayComponents;

impl HasRelayChains for CosmosRelay {
    type SrcChain = CosmosChain;

    type DstChain = CosmosChain;

    type Packet = Packet;

    fn src_chain(&self) -> &CosmosChain {
        &self.src_chain
    }

    fn dst_chain(&self) -> &CosmosChain {
        &self.dst_chain
    }

    fn src_client_id(&self) -> &ClientId {
        &self.src_client_id
    }

    fn dst_client_id(&self) -> &ClientId {
        &self.dst_client_id
    }
}

impl ProvideRuntime<CosmosRelay> for CosmosRelayComponents {
    fn runtime(relay: &CosmosRelay) -> &HermesRuntime {
        &relay.runtime
    }
}
