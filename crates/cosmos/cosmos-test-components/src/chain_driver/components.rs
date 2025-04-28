use cgp::prelude::*;
use hermes_core::test_components::chain_driver::impls::WaitChainReachHeight;
use hermes_core::test_components::chain_driver::traits::{
    ChainStartupWaiterComponent, RandomAmountGeneratorComponent,
};

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
