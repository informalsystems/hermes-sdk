use core::marker::PhantomData;

use cgp::core::field::Index;
use hermes_prelude::*;
use hermes_test_components::chain::traits::{
    CanInstantiateWasmContract, CanUploadWasmContract, HasWalletType,
};
use hermes_test_components::chain_driver::traits::{HasDenom, HasWallet, StakingDenom, UserWallet};
use hermes_test_components::relay_driver::run::CanRunRelayerInBackground;
use hermes_test_components::test_case::traits::test_case::TestCase;

use crate::traits::CanUseBinaryTestDriverMethods;

pub struct TestUploadWasm<A = Index<0>, B = Index<1>>(pub PhantomData<(A, B)>);

impl<A, B> Default for TestUploadWasm<A, B> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Driver, A, B> TestCase<Driver> for TestUploadWasm<A, B>
where
    Driver: CanUseBinaryTestDriverMethods<A, B>,
    A: Async,
    B: Async,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let chain_driver_a = driver.chain_driver_a();

        let user_wallet = chain_driver_a.wallet(PhantomData::<UserWallet<0>>);

        let user_address = Driver::ChainA::wallet_address(user_wallet);

        let relay_driver = driver.relay_driver();

        let chain_a = driver.chain_a();

        let _handle = relay_driver
            .run_relayer_in_background()
            .await
            .map_err(Driver::raise_error)?;

        let code_ids = chain_a
            .upload_wasm_contract(&alloc::vec![], user_address)
            .await
            .unwrap();

        let denom_a = chain_driver_a.denom(PhantomData::<StakingDenom>);

        for code_id in code_ids.iter() {
            let _ = chain_a
                .instantiate_wasm_contract(user_address, user_address, *code_id, denom_a)
                .await;
        }

        Ok(())
    }
}
