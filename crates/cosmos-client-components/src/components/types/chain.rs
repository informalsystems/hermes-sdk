use alloc::sync::Arc;
use cgp_core::Async;
use ibc_relayer_components::chain::traits::types::height::HeightTypeProvider;
use ibc_relayer_components::chain::traits::types::message::MessageTypeProvider;
use ibc_relayer_types::Height;

use crate::traits::message::CosmosMessage;

pub struct ProvideCosmosChainTypes;

impl<Chain> HeightTypeProvider<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Height = Height;
}

impl<Chain> MessageTypeProvider<Chain> for ProvideCosmosChainTypes
where
    Chain: Async,
{
    type Message = Arc<dyn CosmosMessage>;
}
