use cgp_core::prelude::*;
use eyre::eyre;
use hermes_relayer_components::transaction::traits::components::tx_fee_estimater::TxFeeEstimator;
use ibc_proto::cosmos::tx::v1beta1::{Fee, Tx};
use ibc_relayer::chain::cosmos::gas::gas_amount_to_fee;
use ibc_relayer::chain::cosmos::simulate::send_tx_simulate;
use ibc_relayer::chain::cosmos::types::tx::SignedTx;

use crate::contexts::transaction::CosmosTxContext;
use crate::impls::transaction::component::CosmosTxComponents;
use crate::types::error::Error;

#[async_trait]
impl TxFeeEstimator<CosmosTxContext> for CosmosTxComponents {
    async fn estimate_tx_fee(context: &CosmosTxContext, tx: &SignedTx) -> Result<Fee, Error> {
        let tx = Tx {
            body: Some(tx.body.clone()),
            auth_info: Some(tx.auth_info.clone()),
            signatures: tx.signatures.clone(),
        };

        let tx_config = &context.tx_config;

        let response = send_tx_simulate(&tx_config.grpc_address, tx).await?;

        let gas_info = response
            .gas_info
            .ok_or_else(|| eyre!("missing simulate gas info"))?;

        let fee = gas_amount_to_fee(
            &tx_config.gas_config,
            gas_info.gas_used,
            &tx_config.chain_id,
            &tx_config.rpc_address,
        )
        .await;

        Ok(fee)
    }
}
