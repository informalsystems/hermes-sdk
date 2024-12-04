use cgp::core::error::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::transaction::traits::estimate_tx_fee::TxFeeEstimator;
use hermes_relayer_components::transaction::traits::types::fee::HasFeeType;
use hermes_relayer_components::transaction::traits::types::transaction::HasTransactionType;
use http::uri::InvalidUri;
use ibc_proto::cosmos::tx::v1beta1::service_client::ServiceClient;
use ibc_proto::cosmos::tx::v1beta1::{Fee, SimulateRequest, SimulateResponse, Tx};
use ibc_relayer::chain::cosmos::types::tx::SignedTx;
use ibc_relayer::config::default::max_grpc_decoding_size;
use ibc_relayer::error::Error as RelayerError;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use prost::Message;
use tonic::codegen::http::Uri;
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::traits::convert_gas_to_fee::CanConvertGasToFee;
use crate::traits::gas_config::HasGasConfig;
use crate::traits::grpc_address::HasGrpcAddress;
use crate::traits::rpc_client::HasRpcClient;

pub struct EstimateCosmosTxFee;

impl<Chain> TxFeeEstimator<Chain> for EstimateCosmosTxFee
where
    Chain: HasTransactionType<Transaction = SignedTx>
        + HasFeeType<Fee = Fee>
        + HasGrpcAddress
        + HasGasConfig
        + HasRpcClient
        + HasChainId<ChainId = ChainId>
        + CanRaiseError<TransportError>
        + CanRaiseError<Status>
        + CanRaiseError<InvalidUri>
        + CanConvertGasToFee
        + CanRaiseError<&'static str>,
{
    async fn estimate_tx_fee(chain: &Chain, tx: &SignedTx) -> Result<Fee, Chain::Error> {
        let tx = Tx {
            body: Some(tx.body.clone()),
            auth_info: Some(tx.auth_info.clone()),
            signatures: tx.signatures.clone(),
        };

        let tx_bytes = Message::encode_to_vec(&tx);

        let request = SimulateRequest {
            tx_bytes,
            ..Default::default()
        };

        let mut client = ServiceClient::connect(
            Uri::try_from(&chain.grpc_address().to_string()).map_err(Chain::raise_error)?,
        )
        .await
        .map_err(Chain::raise_error)?
        .max_decoding_message_size(max_grpc_decoding_size().get_bytes() as usize);

        let response = client
            .simulate(request)
            .await
            .map_err(Chain::raise_error)?
            .into_inner();

        let gas_info = response
            .gas_info
            .ok_or_else(|| Chain::raise_error("missing simulate gas info"))?;

        let fee = chain.gas_amount_to_fee(gas_info.gas_used).await?;

        Ok(fee)
    }
}

pub async fn send_tx_simulate(
    grpc_address: &Uri,
    tx: Tx,
) -> Result<SimulateResponse, RelayerError> {
    let mut tx_bytes = vec![];
    prost::Message::encode(&tx, &mut tx_bytes)
        .map_err(|e| RelayerError::protobuf_encode(String::from("Transaction"), e))?;

    let req = SimulateRequest {
        tx_bytes,
        ..Default::default()
    };

    let mut client = ServiceClient::connect(grpc_address.clone())
        .await
        .map_err(RelayerError::grpc_transport)?;

    client = client.max_decoding_message_size(max_grpc_decoding_size().get_bytes() as usize);

    let request = tonic::Request::new(req);
    let response = client
        .simulate(request)
        .await
        .map_err(|e| RelayerError::grpc_status(e, "send_tx_simulate".to_owned()))?
        .into_inner();

    Ok(response)
}
