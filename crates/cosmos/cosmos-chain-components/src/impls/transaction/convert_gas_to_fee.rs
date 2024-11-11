use cgp::prelude::CanRaiseError;
use hermes_chain_type_components::traits::fields::chain_id::HasChainId;
use hermes_relayer_components::transaction::traits::{
    convert_gas_to_fee::GasToFeeConverter, types::fee::HasFeeType,
};
use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::chain::cosmos::gas::gas_amount_to_fee;
use ibc_relayer::config::dynamic_gas::DynamicGasPrice;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::traits::gas_config::HasGasConfig;
use crate::traits::rpc_client::HasRpcClient;

pub struct StaticConvertCosmosGasToFee;

impl<Chain> GasToFeeConverter<Chain> for StaticConvertCosmosGasToFee
where
    Chain: HasFeeType<Fee = Fee>
        + HasChainId<ChainId = ChainId>
        + HasRpcClient
        + HasGasConfig
        + CanRaiseError<&'static str>,
{
    async fn gas_amount_to_fee(chain: &Chain, gas_used: u64) -> Result<Chain::Fee, Chain::Error> {
        Ok(gas_amount_to_fee(
            chain.gas_config(),
            gas_used,
            chain.chain_id(),
            chain.rpc_address(),
        )
        .await)
    }
}

pub struct DynamicConvertCosmosGasToFee;

impl<Chain> GasToFeeConverter<Chain> for DynamicConvertCosmosGasToFee
where
    Chain: HasFeeType<Fee = Fee>
        + HasChainId<ChainId = ChainId>
        + HasRpcClient
        + HasGasConfig
        + CanRaiseError<&'static str>,
{
    async fn gas_amount_to_fee(chain: &Chain, gas_used: u64) -> Result<Chain::Fee, Chain::Error> {
        let mut gas_config = chain.gas_config().clone();
        gas_config.dynamic_gas_price = DynamicGasPrice::enabled(1.3, 2.0).unwrap();
        Ok(gas_amount_to_fee(&gas_config, gas_used, chain.chain_id(), chain.rpc_address()).await)
    }
}
