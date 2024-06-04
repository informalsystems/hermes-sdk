use cgp_core::Async;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    CanBuildConnectionOpenAckMessage, CanBuildConnectionOpenConfirmMessage,
    CanBuildConnectionOpenInitMessage, CanBuildConnectionOpenTryMessage,
};
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;

use crate::context::encoding::SolomachineEncoding;
use crate::traits::solomachine::Solomachine;

#[derive(Clone)]
pub struct SolomachineChain<Chain> {
    pub chain: Chain,
}

impl<Chain: Solomachine> SolomachineChain<Chain> {
    pub fn new(chain: Chain) -> Self {
        SolomachineChain { chain }
    }
}

pub trait CanUseSolomachineChain:
    HasEncoding<Encoding = SolomachineEncoding>
    + HasClientStateType<CosmosChain>
    + HasInitConnectionOptionsType<CosmosChain>
    + CanBuildConnectionOpenInitMessage<CosmosChain>
    + CanBuildConnectionOpenTryMessage<CosmosChain>
    + CanBuildConnectionOpenAckMessage<CosmosChain>
    + CanBuildConnectionOpenConfirmMessage<CosmosChain>
    + CanQueryClientState<CosmosChain>
where
    CosmosChain: HasClientStateType<Self>,
{
}

impl<Chain> CanUseSolomachineChain for SolomachineChain<Chain> where Chain: Solomachine {}

pub trait CanQuerySolomachineFromCosmos<Chain>:
    CanQueryClientState<SolomachineChain<Chain>>
where
    Chain: Async,
{
}

impl<Chain> CanQuerySolomachineFromCosmos<Chain> for CosmosChain where Chain: Solomachine {}
