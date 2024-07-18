use core::fmt::Write;
use std::collections::BTreeMap;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use ibc_relayer::keyring::{KeyRing, SigningKeyPair, Store};
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

impl CommandRunner<CosmosBuilder> for KeysListCmd {
    async fn run(&self, builder: &CosmosBuilder) -> hermes_cli_framework::Result<Output> {
        let chain_config = builder
            .config_map
            .get(&self.chain_id)
            .ok_or_else(|| eyre!("chain `{}` not found in configuration file", self.chain_id))?;

        let keyring = KeyRing::new_secp256k1(
            Store::Test,
            &chain_config.account_prefix,
            &chain_config.id,
            &chain_config.key_store_folder,
        )?;

        if json() {
            let keys = keyring.keys()?.into_iter().collect::<BTreeMap<_, _>>();
            Output::success(keys).exit()
        } else {
            let mut msg = String::new();
            for (name, key) in keyring.keys()? {
                let _ = write!(msg, "\n- {} ({})", name, key.account());
            }
            Output::success_msg(msg).exit()
        }
    }
}
