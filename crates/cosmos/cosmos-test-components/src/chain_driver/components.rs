use cgp::prelude::*;
use hermes_test_components::chain_driver::impls::wait::WaitChainReachHeight;
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;
use hermes_test_components::chain_driver::traits::wait::ChainStartupWaiterComponent;

use crate::chain_driver::impls::amount::GenerateRandomAmount;

pub struct CosmosChainDriverComponents;

delegate_components! {
    CosmosChainDriverComponents {
        RandomAmountGeneratorComponent:
            GenerateRandomAmount,
        ChainStartupWaiterComponent:
            WaitChainReachHeight<1>,
    }
}
