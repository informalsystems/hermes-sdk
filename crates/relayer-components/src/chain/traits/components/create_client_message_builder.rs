use cgp_core::prelude::*;
use cgp_core::HasErrorType;

use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::consensus_state::HasConsensusStateType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::message::HasMessageType;
use crate::std_prelude::*;

#[derive_component(CreateClientMessageBuilderComponent, CreateClientMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildCreateClientMessage<Counterparty>: HasMessageType + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self> + HasClientStateType<Self> + HasConsensusStateType<Self>,
{
    async fn build_create_client_message(
        client_state: &Counterparty::ClientState,
        consensus_state: &Counterparty::ConsensusState,
    ) -> Result<Self::Message, Self::Error>;
}
