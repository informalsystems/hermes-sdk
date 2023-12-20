use cgp_core::Async;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::impls::relay::component::CosmosRelayComponents;

impl<SrcChain, DstChain> HasRelayChains for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    type SrcChain = CosmosChain<SrcChain>;

    type DstChain = CosmosChain<DstChain>;

    type Packet = Packet;

    fn src_chain(&self) -> &CosmosChain<SrcChain> {
        &self.src_chain
    }

    fn dst_chain(&self) -> &CosmosChain<DstChain> {
        &self.dst_chain
    }

    fn src_client_id(&self) -> &ClientId {
        &self.src_client_id
    }

    fn dst_client_id(&self) -> &ClientId {
        &self.dst_client_id
    }
}

impl<SrcChain, DstChain> ProvideRuntime<CosmosRelay<SrcChain, DstChain>> for CosmosRelayComponents
where
    SrcChain: Async,
    DstChain: Async,
{
    fn runtime(relay: &CosmosRelay<SrcChain, DstChain>) -> &TokioRuntimeContext {
        &relay.runtime
    }
}
