use core::num::ParseIntError;

use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasAddressType, HasAmountType, HasDenomType};
use hermes_cosmos_chain_components::traits::grpc_address::HasGrpcAddress;
use hermes_test_components::chain::traits::queries::balance::{
    BalanceQuerier, BalanceQuerierComponent,
};
use http::uri::InvalidUri;
use http::Uri;
use ibc_proto::cosmos::bank::v1beta1::query_client::QueryClient;
use ibc_proto::cosmos::bank::v1beta1::QueryBalanceRequest;
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::chain::types::amount::Amount;
use crate::chain::types::denom::Denom;

pub struct QueryCosmosBalance;

#[cgp_provider(BalanceQuerierComponent)]
impl<Chain> BalanceQuerier<Chain> for QueryCosmosBalance
where
    Chain: HasAddressType
        + HasAmountType<Amount = Amount>
        + HasDenomType<Denom = Denom>
        + HasGrpcAddress
        + CanRaiseAsyncError<InvalidUri>
        + CanRaiseAsyncError<TransportError>
        + CanRaiseAsyncError<ParseIntError>
        + CanRaiseAsyncError<InvalidUri>
        + CanRaiseAsyncError<Status>
        + CanRaiseAsyncError<String>,
{
    async fn query_balance(
        chain: &Chain,
        address: &Chain::Address,
        denom: &Denom,
    ) -> Result<Amount, Chain::Error> {
        let mut client = QueryClient::connect(
            Uri::try_from(&chain.grpc_address().to_string()).map_err(Chain::raise_error)?,
        )
        .await
        .map_err(Chain::raise_error)?
        .max_decoding_message_size(33554432);

        let request = tonic::Request::new(QueryBalanceRequest {
            address: address.to_string(),
            denom: denom.to_string(),
        });

        let response = client
            .balance(request)
            .await
            .map(|r| r.into_inner())
            .map_err(Chain::raise_error)?;

        // Querying for a balance might fail, i.e. if the account doesn't actually exist
        let raw_balance = response.balance.ok_or_else(|| {
            Chain::raise_error(format!(
                "queried balance is empty for address {address} and denom {denom}"
            ))
        })?;

        let quantity = raw_balance.amount.parse().map_err(Chain::raise_error)?;

        Ok(Amount {
            quantity,
            denom: denom.clone(),
        })
    }
}
