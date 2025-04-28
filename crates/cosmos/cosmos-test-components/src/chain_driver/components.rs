use hermes_core::test_components::chain_driver::impls::WaitChainReachHeight;
use hermes_core::test_components::chain_driver::traits::{
    ChainStartupWaiterComponent, RandomAmountGeneratorComponent,
};
use hermes_prelude::*;

use crate::chain_driver::impls::GenerateRandomAmount;

pub struct CosmosChainDriverComponents;

delegate_components! {
    CosmosChainDriverComponents {
        RandomAmountGeneratorComponent:
            GenerateRandomAmount,
        ChainStartupWaiterComponent:
            WaitChainReachHeight<1>,
    }
}
