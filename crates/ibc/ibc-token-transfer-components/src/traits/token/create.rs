use cgp::prelude::*;

use hermes_chain_type_components::traits::types::denom::HasDenomType;

#[derive_component(TokenCreatorComponent, TokenCreator<Chain>)]
#[async_trait]
pub trait CanCreateToken: HasDenomType + HasErrorType {
    async fn create_token(&self) -> Result<Self::Denom, Self::Error>;
}
