use cgp_core::{Async, HasComponents};
use ibc_relayer_components_extra::components::extra::chain::ExtraChainComponents;

use crate::contexts::chain::CosmosChain;

pub struct CosmosChainComponents;

impl<Chain> HasComponents for CosmosChain<Chain>
where
    Chain: Async,
{
    type Components = ExtraChainComponents<CosmosChainComponents>;
}
