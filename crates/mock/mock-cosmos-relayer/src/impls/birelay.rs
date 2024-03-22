use cgp_core::{CanRaiseError, HasErrorType};
use hermes_relayer_components::birelay::traits::two_way::{
    HasTwoChainTypes, HasTwoWayRelay, HasTwoWayRelayTypes,
};
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{HasRuntime, HasRuntimeType};

use crate::contexts::birelay::MockCosmosBiRelay;
use crate::contexts::chain::MockCosmosContext;
use crate::contexts::relay::MockCosmosRelay;
use crate::traits::endpoint::BasecoinEndpoint;
use crate::types::error::Error;

impl<SrcChain, DstChain> HasErrorType for MockCosmosBiRelay<SrcChain, DstChain>
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    type Error = Error;
}

impl<SrcChain, DstChain> CanRaiseError<Error> for MockCosmosBiRelay<SrcChain, DstChain>
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    fn raise_error(e: Error) -> Error {
        e
    }
}

impl<SrcChain, DstChain> HasRuntimeType for MockCosmosBiRelay<SrcChain, DstChain>
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    type Runtime = HermesRuntime;
}

impl<SrcChain, DstChain> HasRuntime for MockCosmosBiRelay<SrcChain, DstChain>
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    fn runtime(&self) -> &Self::Runtime {
        &self.runtime
    }
}

impl<SrcChain, DstChain> CanRaiseError<TokioRuntimeError> for MockCosmosBiRelay<SrcChain, DstChain>
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    fn raise_error(e: TokioRuntimeError) -> Self::Error {
        Error::source(e)
    }
}

impl<ChainA, ChainB> HasTwoChainTypes for MockCosmosBiRelay<ChainA, ChainB>
where
    ChainA: BasecoinEndpoint,
    ChainB: BasecoinEndpoint,
{
    type ChainA = MockCosmosContext<ChainA>;

    type ChainB = MockCosmosContext<ChainB>;
}

impl<SrcChain, DstChain> HasTwoWayRelayTypes for MockCosmosBiRelay<SrcChain, DstChain>
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    type RelayAToB = MockCosmosRelay<SrcChain, DstChain>;

    type RelayBToA = MockCosmosRelay<DstChain, SrcChain>;
}

impl<SrcChain, DstChain> HasTwoWayRelay for MockCosmosBiRelay<SrcChain, DstChain>
where
    SrcChain: BasecoinEndpoint,
    DstChain: BasecoinEndpoint,
{
    fn relay_a_to_b(&self) -> &Self::RelayAToB {
        self.relay_a_to_b()
    }

    fn relay_b_to_a(&self) -> &Self::RelayBToA {
        self.relay_b_to_a()
    }
}
