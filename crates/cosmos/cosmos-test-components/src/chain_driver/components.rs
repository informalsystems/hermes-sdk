use cgp_core::prelude::*;
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;

use crate::chain_driver::impls::amount::GenerateRandomAmount;

pub struct CosmosChainDriverComponents;

delegate_components! {
    CosmosChainDriverComponents {
        RandomAmountGeneratorComponent:
            GenerateRandomAmount,
    }
}
