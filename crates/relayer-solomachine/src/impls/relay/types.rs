use cgp_core::{Async, ErrorRaiser, ProvideErrorType};
use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer_components::relay::traits::chains::HasRelayChains;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_cosmos::contexts::chain::CosmosChain;
use ibc_relayer_cosmos::types::error::Error as CosmosError;
use ibc_relayer_runtime::types::error::TokioRuntimeError;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;
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

impl<Chain> ErrorRaiser<SolomachineRelay<Chain>, TokioRuntimeError> for SolomachineRelayComponents
where
    Chain: Async,
{
    fn raise_error(e: TokioRuntimeError) -> Error {
        BaseError::tokio(e).into()
    }
}

impl<Chain> HasRuntime for SolomachineRelay<Chain>
where
    Chain: Async,
{
    type Runtime = TokioRuntimeContext;

    fn runtime(&self) -> &Self::Runtime {
        todo!()
    }
}

impl<Chain> HasRelayChains for SolomachineRelay<Chain>
where
    Chain: Solomachine<Error = Error>,
{
    type SrcChain = SolomachineChain<Chain>;

    type DstChain = CosmosChain<BaseChainHandle>;

    type Packet = Packet;

    fn src_chain_error(e: Error) -> Error {
        e
    }

    fn dst_chain_error(e: CosmosError) -> Error {
        BaseError::cosmos_chain_error(e).into()
    }

    fn src_client_id(&self) -> &ClientId {
        &self.src_client_id
    }

    fn dst_client_id(&self) -> &ClientId {
        &self.dst_client_id
    }

    fn src_chain(&self) -> &SolomachineChain<Chain> {
        &self.src_chain
    }

    fn dst_chain(&self) -> &CosmosChain<BaseChainHandle> {
        &self.dst_chain
    }
}
