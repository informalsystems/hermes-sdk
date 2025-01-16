use cgp::prelude::*;
use http::uri::InvalidUri;
use http::Uri;
use ibc_proto::cosmos::auth::v1beta1::query_client::QueryClient;
use ibc_proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use prost::{DecodeError, Message};
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::traits::grpc_address::HasGrpcAddress;

/// TODO: Move this to a HasAccount component
/// and create a CanQueryAccount trait

#[derive(Clone, Debug)]
pub struct Account {
    pub address: String,
    pub number: u64,
    pub sequence: u64,
}

pub async fn query_account<Chain>(
    chain: &Chain,
    account_address: String,
) -> Result<Account, Chain::Error>
where
    Chain: HasGrpcAddress
        + CanRaiseAsyncError<DecodeError>
        + CanRaiseAsyncError<TransportError>
        + CanRaiseAsyncError<InvalidUri>
        + CanRaiseAsyncError<Status>
        + CanRaiseAsyncError<String>,
{
    let mut client = QueryClient::connect(
        Uri::try_from(&chain.grpc_address().to_string()).map_err(Chain::raise_error)?,
    )
    .await
    .map_err(Chain::raise_error)?;

    let request = tonic::Request::new(QueryAccountRequest {
        address: account_address.clone(),
    });

    let response = client.account(request).await.map_err(Chain::raise_error)?;

    // Querying for an account might fail, i.e. if the account doesn't actually exist
    let resp_account = response
        .into_inner()
        .account
        .ok_or_else(|| format!("empty account for address `{}`", account_address))
        .map_err(Chain::raise_error)?;

    let base_account = if resp_account.type_url == "/cosmos.auth.v1beta1.BaseAccount" {
        BaseAccount::decode(resp_account.value.as_slice()).map_err(Chain::raise_error)?
    } else {
        return Err(Chain::raise_error(format!(
            "unknown account with type_url `{}`",
            resp_account.type_url
        )));
    };

    Ok(Account {
        address: base_account.address,
        number: base_account.account_number,
        sequence: base_account.sequence,
    })
}
