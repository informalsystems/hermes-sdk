use core::str::FromStr;
use core::time::Duration;

use eyre::Report;
use http::Uri;
use ibc::primitives::proto::Any;
use ibc_relayer::chain::cosmos::config::CosmosSdkConfig;
use ibc_relayer::config::types::{MaxMsgNum, MaxTxSize};
use ibc_relayer::config::AddressType;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use tendermint_rpc::Url;

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

impl<'a> TryFrom<&'a CosmosSdkConfig> for TxConfig {
    type Error = Report;

    fn try_from(config: &'a CosmosSdkConfig) -> Result<Self, Self::Error> {
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
            .map(|opt| opt.to_any())
            .collect::<Result<_, _>>()?;

        Ok(Self {
            chain_id: config.id.clone(),
            gas_config,
            rpc_address: config.rpc_addr.clone(),
            grpc_address,
            rpc_timeout: config.rpc_timeout,
            address_type: config.address_type.clone(),
            max_msg_num: config.max_msg_num,
            max_tx_size: config.max_tx_size,
            extension_options,
        })
    }
}
