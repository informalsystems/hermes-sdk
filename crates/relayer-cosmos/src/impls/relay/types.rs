use cgp_core::{Async, HasErrorType};
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_all_in_one::one_for_all::types::chain::OfaChainWrapper;
use ibc_relayer_components::logger::traits::has_logger::{HasLogger, HasLoggerType};
use ibc_relayer_components::relay::traits::chains::HasRelayChains;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_runtime::types::error::Error as TokioError;
use ibc_relayer_runtime::types::log::logger::TracingLogger;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::types::error::{BaseError, Error};

impl<SrcChain, DstChain> HasRelayChains for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    type SrcChain = OfaChainWrapper<CosmosChain<SrcChain>>;

    type DstChain = OfaChainWrapper<CosmosChain<DstChain>>;

    type Packet = Packet;

    fn src_chain_error(e: Error) -> Error {
        e
    }

    fn dst_chain_error(e: Error) -> Error {
        e
    }

    fn src_chain(&self) -> &OfaChainWrapper<CosmosChain<SrcChain>> {
        &self.src_chain
    }

    fn dst_chain(&self) -> &OfaChainWrapper<CosmosChain<DstChain>> {
        &self.dst_chain
    }

    fn src_client_id(&self) -> &ClientId {
        &self.src_client_id
    }

    fn dst_client_id(&self) -> &ClientId {
        &self.dst_client_id
    }
}

impl<SrcChain, DstChain> HasErrorType for CosmosRelay<SrcChain, DstChain>
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

    fn runtime_error(e: TokioError) -> Error {
        BaseError::tokio(e).into()
    }
}

impl<SrcChain, DstChain> HasLoggerType for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: Async,
    DstChain: Async,
{
    type Logger = TracingLogger;
}

impl<SrcChain, DstChain> HasLogger for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: Async,
    DstChain: Async,
{
    fn logger(&self) -> &TracingLogger {
        &TracingLogger
    }
}
