use cgp_core::{Async, ProvideErrorType};
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_components::relay::traits::chains::HasRelayChains;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_runtime::types::error::TokioRuntimeError;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::impls::relay::component::CosmosRelayComponents;
use crate::types::error::{BaseError, Error};

impl<SrcChain, DstChain> HasRelayChains for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    type SrcChain = CosmosChain<SrcChain>;

    type DstChain = CosmosChain<DstChain>;

    type Packet = Packet;

    fn src_chain_error(e: Error) -> Error {
        e
    }

    fn dst_chain_error(e: Error) -> Error {
        e
    }

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

impl<SrcChain, DstChain> ProvideErrorType<CosmosRelay<SrcChain, DstChain>> for CosmosRelayComponents
where
    SrcChain: Async,
    DstChain: Async,
{
    type Error = Error;
}

impl<SrcChain, DstChain> HasRuntime for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: Async,
    DstChain: Async,
{
    type Runtime = TokioRuntimeContext;

    fn runtime(&self) -> &TokioRuntimeContext {
        &self.runtime
    }

    fn runtime_error(e: TokioRuntimeError) -> Error {
        BaseError::tokio(e).into()
    }
}
