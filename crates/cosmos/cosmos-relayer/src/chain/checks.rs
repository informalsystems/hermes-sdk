use hermes_test_components::chain::traits::messages::ibc_transfer::CanBuildIbcTokenTransferMessage;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;

use crate::contexts::chain::CosmosChain;

pub trait CheckCosmosChainImpls:
    CanQueryBalance + CanIbcTransferToken<CosmosChain> + CanBuildIbcTokenTransferMessage<CosmosChain>
{
}

impl CheckCosmosChainImpls for CosmosChain {}
