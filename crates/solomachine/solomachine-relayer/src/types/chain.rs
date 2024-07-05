use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    CanBuildConnectionOpenAckMessage, CanBuildConnectionOpenConfirmMessage,
    CanBuildConnectionOpenInitMessage, CanBuildConnectionOpenTryMessage,
};
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;

use crate::context::chain::MockSolomachine;
use crate::context::encoding::SolomachineEncoding;

pub trait CanUseSolomachineChain:
    HasEncoding<Encoding = SolomachineEncoding>
    + HasClientStateType<CosmosChain>
    + HasInitConnectionOptionsType<CosmosChain>
    + CanBuildConnectionOpenInitMessage<CosmosChain>
    + CanBuildConnectionOpenTryMessage<CosmosChain>
    + CanBuildConnectionOpenAckMessage<CosmosChain>
    + CanBuildConnectionOpenConfirmMessage<CosmosChain>
    + CanQueryClientState<CosmosChain>
{
}

impl CanUseSolomachineChain for MockSolomachine {}

pub trait CanQuerySolomachineFromCosmos: CanQueryClientState<MockSolomachine> {}

impl CanQuerySolomachineFromCosmos for CosmosChain {}
