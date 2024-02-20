use cgp_core::prelude::*;
use hermes_test_components::chain_driver::impls::default_assert_duration::ProvideDefaultPollAssertDuration;
use hermes_test_components::chain_driver::impls::poll_assert_eventual_amount::PollAssertEventualAmount;
use hermes_test_components::chain_driver::traits::assert::eventual_amount::EventualAmountAsserterComponent;
use hermes_test_components::chain_driver::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;

use crate::chain_driver::impls::amount::GenerateRandomAmount;
use crate::chain_driver::impls::store_wasm_client::UploadWasmClientCodeWithChainCommand;
use crate::chain_driver::traits::store_wasm_client::WasmClientCodeUploaderComponent;

pub struct CosmosChainDriverComponents;

delegate_components! {
    CosmosChainDriverComponents {
        RandomAmountGeneratorComponent:
            GenerateRandomAmount,
        EventualAmountAsserterComponent:
            PollAssertEventualAmount,
        PollAssertDurationGetterComponent:
            ProvideDefaultPollAssertDuration,
        WasmClientCodeUploaderComponent:
            UploadWasmClientCodeWithChainCommand,
    }
}
