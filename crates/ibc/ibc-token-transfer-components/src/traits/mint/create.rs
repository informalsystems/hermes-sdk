use cgp::prelude::*;

use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_chain_type_components::traits::types::quantity::HasQuantityType;

#[derive_component(TokenMintCreatorComponent, TokenMintCreator<Chain>)]
#[async_trait]
pub trait CanCreateAndMintToken:
    HasQuantityType + HasAddressType + HasDenomType + HasErrorType
{
    async fn create_and_mint_token(
        &self,
        receiver: &Self::Address,
        quantity: &Self::Quantity,
    ) -> Result<Self::Denom, Self::Error>;
}
