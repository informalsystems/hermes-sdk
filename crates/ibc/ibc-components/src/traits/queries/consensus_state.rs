use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::consensus_state::HasConsensusStateType;

#[async_trait]
pub trait CanQueryConsensusState<Counterparty>:
    HasErrorType + HasChannelIdType<Counterparty>
where
    Counterparty: HasHeightType + HasConsensusStateType<Self>,
{
    async fn query_consensus_state(
        &self,
        channel_id: &Self::ChannelId,
        height: &Counterparty::Height,
    ) -> Result<Counterparty::ConsensusState, Self::Error>;
}
