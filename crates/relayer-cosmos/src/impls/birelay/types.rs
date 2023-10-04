use cgp_core::{Async, HasErrorType};
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_components::logger::traits::has_logger::{HasLogger, HasLoggerType};
use ibc_relayer_components::relay::traits::two_way::HasTwoWayRelay;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_runtime::types::error::Error as TokioError;
use ibc_relayer_runtime::types::log::logger::TracingLogger;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::relay::CosmosRelay;
use crate::types::error::{BaseError, Error};

impl<ChainA, ChainB> HasTwoWayRelay for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: ChainHandle,
    ChainB: ChainHandle,
{
    type RelayAToB = CosmosRelay<ChainA, ChainB>;

    type RelayBToA = CosmosRelay<ChainB, ChainA>;

    fn relay_a_to_b(&self) -> &CosmosRelay<ChainA, ChainB> {
        &self.relay_a_to_b.relay
    }

    fn relay_b_to_a(&self) -> &CosmosRelay<ChainB, ChainA> {
        &self.relay_b_to_a.relay
    }

    fn relay_error(e: Error) -> Error {
        e
    }
}

impl<ChainA, ChainB> HasErrorType for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: Async,
    ChainB: Async,
{
    type Error = Error;
}

impl<ChainA, ChainB> HasRuntime for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: Async,
    ChainB: Async,
{
    type Runtime = TokioRuntimeContext;

    fn runtime(&self) -> &TokioRuntimeContext {
        &self.runtime
    }

    fn runtime_error(e: TokioError) -> Error {
        BaseError::tokio(e).into()
    }
}

impl<ChainA, ChainB> HasLoggerType for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: Async,
    ChainB: Async,
{
    type Logger = TracingLogger;
}

impl<ChainA, ChainB> HasLogger for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: Async,
    ChainB: Async,
{
    fn logger(&self) -> &TracingLogger {
        &TracingLogger
    }
}
