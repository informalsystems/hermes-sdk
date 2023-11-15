use cgp_core::{Async, HasErrorType};
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_components::relay::traits::two_way::{
    HasTwoChainTypes, HasTwoWayRelay, HasTwoWayRelayTypes,
};
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_runtime::types::error::Error as TokioError;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::types::error::{BaseError, Error};

impl<ChainA, ChainB> HasTwoChainTypes for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: ChainHandle,
    ChainB: ChainHandle,
{
    type ChainA = CosmosChain<ChainA>;

    type ChainB = CosmosChain<ChainB>;
}

impl<ChainA, ChainB> HasTwoWayRelayTypes for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: ChainHandle,
    ChainB: ChainHandle,
{
    type RelayAToB = CosmosRelay<ChainA, ChainB>;

    type RelayBToA = CosmosRelay<ChainB, ChainA>;
}

impl<ChainA, ChainB> HasTwoWayRelay for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: ChainHandle,
    ChainB: ChainHandle,
{
    fn relay_a_to_b(&self) -> &CosmosRelay<ChainA, ChainB> {
        &self.relay_a_to_b
    }

    fn relay_b_to_a(&self) -> &CosmosRelay<ChainB, ChainA> {
        &self.relay_b_to_a
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
