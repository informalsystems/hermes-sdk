use core::fmt::Write;
use std::collections::BTreeMap;

use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use ibc::core::host::types::identifiers::ChainId;
use ibc_relayer::keyring::{KeyRing, SigningKeyPair, Store};
use oneline_eyre::eyre::eyre;

use crate::contexts::app::HermesApp;

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

impl CommandRunner<HermesApp> for KeysListCmd {
    async fn run(&self, app: &HermesApp) -> hermes_cli_framework::Result<Output> {
        let builder = app.load_builder().await?;

        let chain_config = builder
            .config_map
            .get(&self.chain_id)
            .ok_or_else(|| eyre!("chain `{}` not found in configuration file", self.chain_id))?;

        let keyring = KeyRing::new_secp256k1(
            Store::Test,
            &chain_config.account_prefix,
            &ChainId::new(&chain_config.id)?.to_string().into(),
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
