use cgp_core::CanRaiseError;
use hermes_sovereign_client_components::sovereign::traits::rollup::json_rpc_client::HasJsonRpcClient;
use hermes_test_components::chain::traits::queries::balance::BalanceQuerier;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain::traits::types::denom::HasDenomType;
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

impl<Rollup> BalanceQuerier<Rollup> for QuerySovereignBalance
where
    Rollup: HasAddressType
        + HasDenomType<Denom = String>
        + HasAmountType<Amount = SovereignAmount>
        + CanRaiseError<ClientError>
        + CanRaiseError<serde_json::Error>
        + HasJsonRpcClient,
{
    async fn query_balance(
        rollup: &Rollup,
        address: &Rollup::Address,
        denom: &Rollup::Denom,
    ) -> Result<SovereignAmount, Rollup::Error> {
        let rpc_client = rollup.json_rpc_client();

        let mut params = ArrayParams::new();

        params.insert(None::<u64>).map_err(Rollup::raise_error)?;

        params
            .insert(address.to_string())
            .map_err(Rollup::raise_error)?;

        params
            .insert(denom.to_string())
            .map_err(Rollup::raise_error)?;

        let response: Response = rpc_client
            .request("bank_balanceOf", params)
            .await
            .map_err(Rollup::raise_error)?;

        let amount = SovereignAmount {
            quantity: response.amount,
            denom: denom.clone(),
        };

        Ok(amount)
    }
}
