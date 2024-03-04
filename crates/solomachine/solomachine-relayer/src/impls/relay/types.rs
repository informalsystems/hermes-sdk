use cgp_core::Async;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::types::error::Error;
use hermes_relayer_components::relay::traits::chains::ProvideRelayChains;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::context::relay::SolomachineRelay;
use crate::impls::relay::component::SolomachineRelayComponents;
use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;

impl<Chain> ProvideRuntime<SolomachineRelay<Chain>> for SolomachineRelayComponents
where
    Chain: Async,
{
    fn runtime(relay: &SolomachineRelay<Chain>) -> &HermesRuntime {
        &relay.runtime
    }
}

impl<Chain> ProvideRelayChains<SolomachineRelay<Chain>> for SolomachineRelayComponents
where
    Chain: Solomachine<Error = Error>,
{
    type SrcChain = SolomachineChain<Chain>;

    type DstChain = CosmosChain;

    type Packet = Packet;

    fn src_client_id(relay: &SolomachineRelay<Chain>) -> &ClientId {
        &relay.src_client_id
    }

    fn dst_client_id(relay: &SolomachineRelay<Chain>) -> &ClientId {
        &relay.dst_client_id
    }

    fn src_chain(relay: &SolomachineRelay<Chain>) -> &SolomachineChain<Chain> {
        &relay.src_chain
    }

    fn dst_chain(relay: &SolomachineRelay<Chain>) -> &CosmosChain {
        &relay.dst_chain
    }
}
