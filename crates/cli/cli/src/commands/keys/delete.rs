use eyre::eyre;
use hermes_cli_components::traits::{CanLoadBuilder, CommandRunnerComponent};
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos::chain_components::impls::CosmosChainConfig;
use hermes_cosmos::chain_components::types::KeyRing;
use hermes_cosmos::ibc::core::host::types::identifiers::ChainId;
use hermes_prelude::*;
use tracing::warn;

use crate::contexts::HermesApp;

#[derive(Debug, clap::Parser)]
#[clap(
    override_usage = "hermes keys delete --chain <CHAIN_ID> --key-name <KEY_NAME>

    hermes keys delete --chain <CHAIN_ID> --all"
)]
pub struct KeysDeleteCmd {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "FLAGS",
        help = "Identifier of the chain"
    )]
    chain_id: ChainId,

    #[clap(
        long = "key-name",
        required = true,
        value_name = "KEY_NAME",
        group = "delete_mode",
        help_heading = "FLAGS",
        help = "Name of the key"
    )]
    key_name: Option<String>,

    #[clap(
        long = "all",
        required = true,
        group = "delete_mode",
        help_heading = "FLAGS",
        help = "Delete all keys"
    )]
    all: bool,
}

impl KeysDeleteCmd {
    fn options(&self, config: &CosmosChainConfig) -> eyre::Result<KeysDeleteOptions<'_>> {
        let id = match (self.all, &self.key_name) {
            (true, None) => KeysDeleteId::All,
            (false, Some(ref key_name)) => KeysDeleteId::Named(key_name),
            // This case should never trigger.
            // The 'required' parameter for the flags will trigger an error if both flags have not been given.
            // And the 'group' parameter for the flags will trigger an error if both flags are given.
            _ => Output::error("--key-name and --all can't both be set or both None".to_string())
                .exit(),
        };

        Ok(KeysDeleteOptions {
            config: config.clone(),
            id,
        })
    }
}

#[derive(Clone, Debug)]
struct KeysDeleteOptions<'a> {
    id: KeysDeleteId<'a>,
    config: CosmosChainConfig,
}

#[derive(Clone, Debug)]
enum KeysDeleteId<'a> {
    All,
    Named(&'a str),
}

#[cgp_provider(CommandRunnerComponent)]
impl CommandRunner<HermesApp> for KeysDeleteCmd {
    async fn run(&self, app: &HermesApp) -> hermes_cli_framework::Result<Output> {
        let builder = app.load_builder().await?;

        let chain_config = builder
            .config_map
            .get(&self.chain_id)
            .ok_or_else(|| eyre!("chain `{}` not found in configuration file", self.chain_id))?;

        let opts = match self.options(chain_config) {
            Err(e) => Output::error(e).exit(),
            Ok(opts) => opts,
        };

        match opts.id {
            KeysDeleteId::All => match delete_all_keys(&opts.config) {
                Ok(_) => {
                    Output::success(format!("Removed all keys on chain `{}`", opts.config.id,))
                        .exit()
                }
                Err(e) => Output::error(format!(
                    "`keys delete` command failed to delete all keys: {e}"
                ))
                .exit(),
            },
            KeysDeleteId::Named(key_name) => match delete_key(&opts.config, key_name) {
                Ok(_) => Output::success_msg(format!(
                    "Removed key `{key_name}` on chain `{}`",
                    opts.config.id
                ))
                .exit(),
                Err(e) => Output::error(format!(
                    "`keys delete` command failed to delete key `{key_name}`: {e}"
                ))
                .exit(),
            },
        }
    }
}

fn delete_key(config: &CosmosChainConfig, key_name: &str) -> eyre::Result<()> {
    let mut keyring = KeyRing::new_secp256k1(
        &config.account_prefix,
        &ChainId::new(&config.id)?,
        &config.key_store_folder,
    );

    keyring.remove_key(key_name).map_err(|e| eyre!("{e}"))?;

    Ok(())
}

fn delete_all_keys(config: &CosmosChainConfig) -> eyre::Result<()> {
    let mut keyring = KeyRing::new_secp256k1(
        &config.account_prefix,
        &ChainId::new(&config.id)?,
        &config.key_store_folder,
    );

    let keys = keyring.keys().map_err(|e| eyre!("{e}"))?;

    for (key_name, _) in keys {
        if let Err(e) = keyring.remove_key(&key_name) {
            warn!("failed to remove key `{key_name}` from keyring: {e}");
            continue;
        }
    }

    Ok(())
}
