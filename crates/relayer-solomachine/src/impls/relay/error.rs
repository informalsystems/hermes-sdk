use eyre::eyre;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc_relayer_components::relay::components::create_client::CanRaiseMissingCreateClientEventError;
use ibc_relayer_components::relay::impls::connection::open_init::CanRaiseMissingConnectionInitEventError;
use ibc_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};

use crate::context::relay::SolomachineRelay;
use crate::traits::solomachine::Solomachine;
use crate::types::error::{BaseError, Error};

impl<Chain> CanRaiseMissingCreateClientEventError<SourceTarget> for SolomachineRelay<Chain>
where
    Chain: Solomachine<Error = Error>,
{
    fn missing_create_client_event_error(
        src_chain: &Self::SrcChain,
        dst_chain: &Self::DstChain,
    ) -> Self::Error {
        BaseError::generic(eyre!("missing CreateClient event when creating client from chain {} with counterparty chain {}",
            src_chain.chain_id(),
            dst_chain.chain_id(),
        )).into()
    }
}

impl<Chain> CanRaiseMissingCreateClientEventError<DestinationTarget> for SolomachineRelay<Chain>
where
    Chain: Solomachine<Error = Error>,
{
    fn missing_create_client_event_error(
        dst_chain: &Self::DstChain,
        src_chain: &Self::SrcChain,
    ) -> Self::Error {
        BaseError::generic(eyre!("missing CreateClient event when creating client from chain {} with counterparty chain {}",
            dst_chain.chain_id(),
            src_chain.chain_id(),
        )).into()
    }
}

impl<Chain> CanRaiseMissingConnectionInitEventError for SolomachineRelay<Chain>
where
    Chain: Solomachine<Error = Error>,
{
    fn missing_connection_init_event_error(&self) -> Self::Error {
        BaseError::generic(eyre!("missing ConnectionOpenInit event")).into()
    }
}
