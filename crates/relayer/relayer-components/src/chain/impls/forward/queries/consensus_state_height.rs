use cgp_core::{CanRaiseError, HasInner};

use crate::chain::traits::queries::consensus_state_height::{
    CanQueryConsensusStateHeight, ConsensusStateHeightQuerier,
};
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub struct ForwardQueryConsensusStateHeight;

impl<Chain, InChain, Counterparty> ConsensusStateHeightQuerier<Chain, Counterparty>
    for ForwardQueryConsensusStateHeight
where
    Chain:
        HasIbcChainTypes<Counterparty> + HasInner<Inner = InChain> + CanRaiseError<InChain::Error>,
    Counterparty: HasHeightType,
    InChain: CanQueryConsensusStateHeight<Counterparty, ClientId = Chain::ClientId>,
{
    async fn find_consensus_state_height_before(
        chain: &Chain,
        client_id: &Chain::ClientId,
        target_height: &Counterparty::Height,
    ) -> Result<Counterparty::Height, Chain::Error> {
        chain
            .inner()
            .find_consensus_state_height_before(client_id, target_height)
            .await
            .map_err(Chain::raise_error)
    }
}
