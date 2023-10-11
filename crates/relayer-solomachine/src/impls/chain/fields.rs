use ibc_relayer_components::chain::traits::types::chain_id::HasChainId;

use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;

impl<Chain> HasChainId for SolomachineChain<Chain>
where
    Chain: Solomachine,
{
    fn chain_id(&self) -> &Self::ChainId {
        self.chain.get_chain_id()
    }
}
