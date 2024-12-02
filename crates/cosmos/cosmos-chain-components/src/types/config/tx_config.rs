use core::str::FromStr;
use core::time::Duration;
use eyre::Report;
use http::Uri;
use ibc::primitives::proto::Any;
use ibc_relayer::extension_options::ExtensionOptionDynamicFeeTx;
use tendermint_rpc::Url;

use ibc_relayer::config::types::{MaxMsgNum, MaxTxSize};
use ibc_relayer::config::AddressType;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::impls::types::config::CosmosChainConfig;
use crate::types::config::gas::gas_config::GasConfig;

pub struct TxConfig {
    pub chain_id: ChainId,
    pub gas_config: GasConfig,
    pub rpc_address: Url,
    pub grpc_address: Uri,
    pub rpc_timeout: Duration,
    pub address_type: AddressType,
    pub max_msg_num: MaxMsgNum,
    pub max_tx_size: MaxTxSize,
    pub extension_options: Vec<Any>,
}

impl<'a> TryFrom<&'a CosmosChainConfig> for TxConfig {
    type Error = Report;

    fn try_from(config: &'a CosmosChainConfig) -> Result<Self, Self::Error> {
        let grpc_address = Uri::from_str(&config.grpc_addr.to_string()).map_err(|e| {
            Report::msg(format!(
                "failed to create Uri from gRPC address string `{}`: {e}",
                config.grpc_addr
            ))
        })?;

        let gas_config = GasConfig::from(config);

        let extension_options = config
            .extension_options
            .iter()
            .map(|opt| {
                ExtensionOptionDynamicFeeTx {
                    max_priority_price: opt.into(),
                }
                .to_any()
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            chain_id: ChainId::from_string(&config.id),
            gas_config,
            rpc_address: Url::from_str(&config.rpc_addr).unwrap(),
            grpc_address,
            rpc_timeout: config.rpc_timeout,
            address_type: AddressType::Cosmos,
            max_msg_num: MaxMsgNum::new(config.max_msg_num).unwrap(),
            max_tx_size: MaxTxSize::new(config.max_tx_size).unwrap(),
            extension_options,
        })
    }
}
