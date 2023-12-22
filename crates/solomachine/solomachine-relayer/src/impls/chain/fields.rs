use hermes_relayer_components::chain::traits::types::chain_id::ChainIdGetter;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::impls::chain::component::SolomachineChainComponents;
use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;

impl<Chain> ChainIdGetter<SolomachineChain<Chain>> for SolomachineChainComponents
where
    Chain: Solomachine,
{
    fn chain_id(chain: &SolomachineChain<Chain>) -> &ChainId {
        chain.chain.get_chain_id()
    }
}
