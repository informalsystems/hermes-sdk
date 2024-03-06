use cgp_core::Async;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::CanBuildConnectionHandshakeMessages;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::encode::traits::has_encoding::HasEncoding;

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
    + HasInitConnectionOptionsType<CosmosChain>
    + CanBuildConnectionHandshakeMessages<CosmosChain>
    + CanQueryClientState<CosmosChain>
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
