use core::fmt::Write;
use std::collections::BTreeMap;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use ibc_relayer::config::{ChainConfig, Config};
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use oneline_eyre::eyre::eyre;

#[derive(Debug, clap::Parser)]
pub struct KeysListCmd {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain"
    )]
    chain_id: ChainId,
}

#[derive(Clone, Debug)]
pub struct KeysListOptions {
    pub chain_config: ChainConfig,
}

impl KeysListCmd {
    fn options(&self, config: &Config) -> eyre::Result<KeysListOptions> {
        let chain_config = config
            .find_chain(&self.chain_id)
            .ok_or_else(|| eyre!("chain `{}` not found in configuration file", self.chain_id))?;

        Ok(KeysListOptions {
            chain_config: chain_config.clone(),
        })
    }
}

impl CommandRunner<CosmosBuilder> for KeysListCmd {
    async fn run(&self, builder: &CosmosBuilder) -> hermes_cli_framework::Result<Output> {
        let config = &builder.config;

        let opts = match self.options(config) {
            Err(e) => Output::error(e).exit(),
            Ok(opts) => opts,
        };

        match &opts.chain_config.list_keys() {
            Ok(keys) if json() => {
                let keys = keys
                    .iter()
                    .map(|(n, k)| (n.to_string(), k.clone()))
                    .collect::<BTreeMap<_, _>>();
                Output::success(keys).exit()
            }
            Ok(keys) => {
                let mut msg = String::new();
                for (name, key) in keys {
                    let _ = write!(msg, "\n- {} ({})", name, key.account());
                }
                Output::success_msg(msg).exit()
            }
            Err(e) => Output::error(format!("`keys list` command failed: {}", e)).exit(),
        }
    }
}
