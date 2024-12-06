use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use hdpath::StandardHDPath;
use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_chain_components::impls::types::config::CosmosChainConfig;
use ibc::core::host::types::identifiers::ChainId;
use ibc_relayer::keyring::{
    AnySigningKeyPair, KeyRing, Secp256k1KeyPair, SigningKeyPair, SigningKeyPairSized, Store,
};
use oneline_eyre::eyre;
use oneline_eyre::eyre::{eyre, WrapErr};
use tracing::warn;

use crate::contexts::app::HermesApp;

/// The data structure that represents the arguments when invoking the `keys add` CLI command.
///
/// The command has one argument and two exclusive flags:
///
/// The command to add a key from a file:
///
/// `keys add [OPTIONS] --chain <CHAIN_ID> --key-file <KEY_FILE>`
///
/// The command to restore a key from a file containing its mnemonic:
///
/// `keys add [OPTIONS] --chain <CHAIN_ID> --mnemonic-file <MNEMONIC_FILE>`
///
/// On *nix platforms, both flags also accept `/dev/stdin` as a value, which will read the key or the mnemonic from stdin.
///
/// The `--key-file` and `--mnemonic-file` flags cannot both be provided at the same time, this will cause a terminating error.
///
/// If successful the key will be created or restored, depending on which flag was given.
#[derive(Debug, clap::Parser)]
#[clap(override_usage = "Add a key from a Comet keyring file:
        hermes keys add [OPTIONS] --chain <CHAIN_ID> --key-file <KEY_FILE>

    Add a key from a file containing its mnemonic:
        hermes keys add [OPTIONS] --chain <CHAIN_ID> --mnemonic-file <MNEMONIC_FILE>

    On *nix platforms, both flags also accept `/dev/stdin` as a value, which will read the key or the mnemonic from stdin.")]
pub struct KeysAddCmd {
    #[clap(
        long = "chain",
        required = true,
        help_heading = "FLAGS",
        help = "Identifier of the chain"
    )]
    chain_id: ChainId,

    #[clap(
        long = "key-file",
        required = true,
        value_name = "KEY_FILE",
        help_heading = "FLAGS",
        help = "Path to the key file, or /dev/stdin to read the content from stdin",
        group = "add-restore"
    )]
    key_file: Option<PathBuf>,

    #[clap(
        long = "mnemonic-file",
        required = true,
        value_name = "MNEMONIC_FILE",
        help_heading = "FLAGS",
        help = "Path to file containing the mnemonic to restore the key from, or /dev/stdin to read the mnemonic from stdin",
        group = "add-restore"
    )]
    mnemonic_file: Option<PathBuf>,

    #[clap(
        long = "key-name",
        value_name = "KEY_NAME",
        help = "Name of the key (defaults to the `key_name` defined in the config)"
    )]
    key_name: Option<String>,

    #[clap(
        long = "hd-path",
        value_name = "HD_PATH",
        help = "Derivation path for this key",
        default_value = "m/44'/118'/0'/0/0"
    )]
    hd_path: String,

    #[clap(
        long = "overwrite",
        help = "Overwrite the key if there is already one with the same key name"
    )]
    overwrite: bool,
}

impl KeysAddCmd {
    fn options(&self, chain_config: &CosmosChainConfig) -> eyre::Result<KeysAddOptions> {
        let name = self
            .key_name
            .clone()
            .unwrap_or_else(|| chain_config.key_name.to_string());

        let hd_path = StandardHDPath::from_str(&self.hd_path)
            .map_err(|_| eyre!("invalid derivation path: {}", self.hd_path))?;

        Ok(KeysAddOptions {
            config: chain_config.clone(),
            name,
            hd_path,
        })
    }
}

#[derive(Clone, Debug)]
pub struct KeysAddOptions {
    pub name: String,
    pub config: CosmosChainConfig,
    pub hd_path: StandardHDPath,
}

pub fn add_key(
    config: &CosmosChainConfig,
    key_name: &str,
    file: &Path,
    hd_path: &StandardHDPath,
    overwrite: bool,
) -> eyre::Result<AnySigningKeyPair> {
    let mut keyring = KeyRing::new_secp256k1(
        Store::Test,
        &config.account_prefix,
        &ChainId::new(&config.id)?.to_string().into(),
        &config.key_store_folder,
    )?;

    check_key_exists(&keyring, key_name, overwrite);

    let key_contents = fs::read_to_string(file).wrap_err("error reading the key file")?;
    let key_pair = Secp256k1KeyPair::from_seed_file(&key_contents, hd_path)?;

    keyring.add_key(key_name, key_pair.clone())?;

    Ok(key_pair.into())
}

pub fn restore_key(
    mnemonic: &Path,
    key_name: &str,
    hdpath: &StandardHDPath,
    config: &CosmosChainConfig,
    overwrite: bool,
) -> eyre::Result<AnySigningKeyPair> {
    let mnemonic_content =
        fs::read_to_string(mnemonic).wrap_err("error reading the mnemonic file")?;

    let mut keyring = KeyRing::new_secp256k1(
        Store::Test,
        &config.account_prefix,
        &ChainId::new(&config.id)?.to_string().into(),
        &config.key_store_folder,
    )?;

    check_key_exists(&keyring, key_name, overwrite);

    let key_pair = Secp256k1KeyPair::from_mnemonic(
        &mnemonic_content,
        hdpath,
        &ibc_relayer::config::AddressType::Cosmos,
        keyring.account_prefix(),
    )?;

    keyring.add_key(key_name, key_pair.clone())?;

    Ok(key_pair.into())
}

/// Check if the key with the given key name already exists.
/// If it already exists and overwrite is false, abort the command with an error.
/// If overwrite is true, output a warning message informing the key will be overwritten.
fn check_key_exists<S: SigningKeyPairSized>(keyring: &KeyRing<S>, key_name: &str, overwrite: bool) {
    if keyring.get_key(key_name).is_ok() {
        if overwrite {
            warn!("key {} will be overwritten", key_name);
        } else {
            Output::error(format!("key with name '{key_name}' already exists")).exit();
        }
    }
}

impl CommandRunner<HermesApp> for KeysAddCmd {
    async fn run(&self, app: &HermesApp) -> hermes_cli_framework::Result<Output> {
        let builder = app.load_builder().await?;

        let chain_config = builder
            .config_map
            .get(&self.chain_id)
            .ok_or_else(|| eyre!("no chain configuration found for chain `{}`", self.chain_id))?;

        let opts = match self.options(chain_config) {
            Err(err) => Output::error(err).exit(),
            Ok(result) => result,
        };

        // Check if --key-file or --mnemonic-file was given as input.
        match (self.key_file.clone(), self.mnemonic_file.clone()) {
            (Some(key_file), _) => {
                let key = add_key(
                    &opts.config,
                    &opts.name,
                    &key_file,
                    &opts.hd_path,
                    self.overwrite,
                );
                match key {
                    Ok(key) => Output::success_msg(format!(
                        "added key '{}' ({}) on chain `{}`",
                        opts.name,
                        key.account(),
                        opts.config.id,
                    ))
                    .exit(),
                    Err(e) => Output::error(format!(
                        "an error occurred adding the key on chain `{}` from file {:?}: {}",
                        self.chain_id, key_file, e
                    ))
                    .exit(),
                }
            }
            (_, Some(mnemonic_file)) => {
                let key = restore_key(
                    &mnemonic_file,
                    &opts.name,
                    &opts.hd_path,
                    &opts.config,
                    self.overwrite,
                );

                match key {
                    Ok(key) => Output::success_msg(format!(
                        "restored key '{}' ({}) on chain `{}`",
                        opts.name,
                        key.account(),
                        opts.config.id
                    ))
                    .exit(),
                    Err(e) => Output::error(format!(
                        "failed to restore the key on chain `{}` from file {:?}: {}",
                        self.chain_id, mnemonic_file, e
                    ))
                    .exit(),
                }
            }
            // This case should never trigger.
            // The 'required' parameter for the flags will trigger an error if both flags have not been given.
            // And the 'group' parameter for the flags will trigger an error if both flags are given.
            _ => Output::error(
                "exactly one of --mnemonic-file and --key-file must be given".to_string(),
            )
            .exit(),
        }
    }
}
