use cgp_core::Async;
use ibc_relayer_components::chain::traits::types::height::HeightTypeProvider;
use ibc_relayer_types::Height;

pub struct ProvideCosmosTypes;

impl<Chain> HeightTypeProvider<Chain> for ProvideCosmosTypes
where
    Chain: Async,
{
    type Height = Height;
}
