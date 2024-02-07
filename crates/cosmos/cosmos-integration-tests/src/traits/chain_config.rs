use cgp_core::prelude::*;
use ibc_relayer::config::ChainConfig;

pub trait CanBuildRelayerChainConfig: HasErrorType {
    fn build_relayer_chain_config(&self) -> Result<ChainConfig, Self::Error>;
}
