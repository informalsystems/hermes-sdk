use core::num::ParseIntError;

use cgp_core::CanRaiseError;
use hermes_cosmos_chain_components::traits::grpc_address::HasGrpcAddress;
use hermes_test_components::chain::traits::queries::balance::BalanceQuerier;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use ibc_relayer::chain::cosmos::query::balance::query_balance;
use ibc_relayer::error::Error as RelayerError;

use crate::chain::types::amount::Amount;
use crate::chain::types::denom::Denom;

pub struct QueryCosmosBalance;

impl<Chain> BalanceQuerier<Chain> for QueryCosmosBalance
where
    Chain: HasAddressType
        + HasAmountType<Amount = Amount, Denom = Denom>
        + HasGrpcAddress
        + CanRaiseError<ParseIntError>
        + CanRaiseError<RelayerError>,
{
    async fn query_balance(
        chain: &Chain,
        address: &Chain::Address,
        denom: &Denom,
    ) -> Result<Amount, Chain::Error> {
        let grpc_address = chain.grpc_address();
        let denom_str = denom.to_string();

        let balance = query_balance(grpc_address, &address.to_string(), &denom_str)
            .await
            .map_err(Chain::raise_error)?;

        let quantity = balance.amount.parse().map_err(Chain::raise_error)?;

        Ok(Amount {
            quantity,
            denom: denom.clone(),
        })
    }
}
