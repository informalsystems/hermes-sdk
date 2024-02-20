use core::num::ParseIntError;

use cgp_core::CanRaiseError;
use hermes_cosmos_client_components::traits::grpc_address::HasGrpcAddress;
use hermes_test_components::chain::traits::queries::balance::BalanceQuerier;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use ibc_relayer::chain::cosmos::query::balance::query_balance;
use ibc_relayer::error::Error as RelayerError;

use crate::chain::types::amount::Amount;
use crate::chain::types::denom::Denom;

pub struct QueryCosmosBalance;

impl<ChainDriver> BalanceQuerier<ChainDriver> for QueryCosmosBalance
where
    ChainDriver: HasAddressType
        + HasChain
        + HasAmountType<Amount = Amount, Denom = Denom>
        + CanRaiseError<ParseIntError>
        + CanRaiseError<RelayerError>,
    ChainDriver::Chain: HasGrpcAddress,
{
    async fn query_balance(
        chain_driver: &ChainDriver,
        address: &ChainDriver::Address,
        denom: &Denom,
    ) -> Result<Amount, ChainDriver::Error> {
        let grpc_address = chain_driver.chain().grpc_address();
        let denom_str = denom.to_string();

        let balance = query_balance(grpc_address, &address.to_string(), &denom_str)
            .await
            .map_err(ChainDriver::raise_error)?;

        let quantity = balance.amount.parse().map_err(ChainDriver::raise_error)?;

        Ok(Amount {
            quantity,
            denom: denom.clone(),
        })
    }
}
