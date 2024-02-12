use cgp_core::CanRaiseError;
use hermes_sovereign_client_components::sovereign::traits::chain::rollup::HasRollup;
use hermes_sovereign_client_components::sovereign::traits::rollup::json_rpc_client::HasJsonRpcClient;
use hermes_test_components::chain_driver::traits::queries::balance::BalanceQuerier;
use hermes_test_components::chain_driver::traits::types::address::HasAddressType;
use hermes_test_components::chain_driver::traits::types::amount::HasAmountType;
use hermes_test_components::chain_driver::traits::types::denom::HasDenomType;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::params::ArrayParams;
use jsonrpsee::core::ClientError;
use serde::Deserialize;

use crate::types::amount::SovereignAmount;

#[derive(Deserialize)]
pub struct Response {
    pub amount: u128,
}

pub struct QuerySovereignBalance;

impl<RollupDriver, Rollup> BalanceQuerier<RollupDriver> for QuerySovereignBalance
where
    RollupDriver: HasAddressType
        + HasDenomType<Denom = String>
        + HasAmountType<Amount = SovereignAmount>
        + HasRollup<Rollup = Rollup>
        + CanRaiseError<ClientError>
        + CanRaiseError<serde_json::Error>,
    Rollup: HasJsonRpcClient,
{
    async fn query_balance(
        rollup_driver: &RollupDriver,
        address: &RollupDriver::Address,
        denom: &RollupDriver::Denom,
    ) -> Result<SovereignAmount, RollupDriver::Error> {
        let rpc_client = rollup_driver.rollup().json_rpc_client();

        let mut params = ArrayParams::new();

        params
            .insert(None::<u64>)
            .map_err(RollupDriver::raise_error)?;

        params
            .insert(address.to_string())
            .map_err(RollupDriver::raise_error)?;

        params
            .insert(denom.to_string())
            .map_err(RollupDriver::raise_error)?;

        let response: Response = rpc_client
            .request("bank_balanceOf", params)
            .await
            .map_err(RollupDriver::raise_error)?;

        let amount = SovereignAmount {
            quantity: response.amount,
            denom: denom.clone(),
        };

        Ok(amount)
    }
}
