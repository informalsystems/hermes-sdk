use cgp_core::{Async, ErrorRaiser, ProvideErrorType};
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::types::error::Error as CosmosError;
use hermes_relayer_components::relay::traits::chains::ProvideRelayChains;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_runtime::types::error::TokioRuntimeError;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::context::relay::SolomachineRelay;
use crate::impls::relay::component::SolomachineRelayComponents;
use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;
use crate::types::error::{BaseError, Error};

impl<Chain> ProvideErrorType<SolomachineRelay<Chain>> for SolomachineRelayComponents
where
    Chain: Async,
{
    type Error = Error;
}

impl<Chain> ErrorRaiser<SolomachineRelay<Chain>, Error> for SolomachineRelayComponents
where
    Chain: Async,
{
    fn raise_error(e: Error) -> Error {
        e
    }
}

impl<Chain> ErrorRaiser<SolomachineRelay<Chain>, CosmosError> for SolomachineRelayComponents
where
    Chain: Async,
{
    fn raise_error(e: CosmosError) -> Error {
        BaseError::cosmos_chain_error(e).into()
    }
}

impl<Chain> ErrorRaiser<SolomachineRelay<Chain>, TokioRuntimeError> for SolomachineRelayComponents
where
    Chain: Async,
{
    fn raise_error(e: TokioRuntimeError) -> Error {
        BaseError::tokio(e).into()
    }
}

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
