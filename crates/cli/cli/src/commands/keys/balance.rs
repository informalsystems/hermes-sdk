use core::fmt::Write;

use ibc_relayer::chain::handle::ChainHandle;
use oneline_eyre::eyre::eyre;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::Result;

/// The data structure that represents the arguments when invoking the `keys balance` CLI command.
///
/// The command has one argument and one optional flag:
///
/// `keys balance --chain <chain_id> --key-name <KEY_NAME>`
///
/// If no key name is given, it will be taken from the configuration file.
/// If successful the balance and denominator of the account, associated with the key name
/// on the given chain, will be displayed.
#[derive(Debug, clap::Parser)]
pub struct KeysBalanceCmd {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain"
    )]
    chain_id: ChainId,

    #[clap(
        long = "key-name",
        value_name = "KEY_NAME",
        help = "(optional) name of the key (defaults to the `key_name` defined in the config)"
    )]
    key_name: Option<String>,

    #[clap(
        long = "denom",
        value_name = "DENOM",
        help = "(optional) query the balance for the given denom (defaults to the `denom` defined in the config for the gas price)"
    )]
    denom: Option<String>,

    #[clap(
        long = "all",
        help = "(optional) query the balance for all denom. This flag overwrites the `--denom` flag (defaults to false)"
    )]
    all: bool,
}

impl CommandRunner<CosmosBuilder> for KeysBalanceCmd {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let config = &builder.config;
        let chain = builder.build_chain(&self.chain_id).await?;

        let key_name = self.key_name.clone();

        if self.all {
            get_balances(chain, key_name)
        } else {
            get_balance(chain, key_name, self.denom.clone())
        }
    }
}

fn get_balance(
    chain: impl ChainHandle,
    key_name: Option<String>,
    denom: Option<String>,
) -> Result<Output> {
    match chain.query_balance(key_name.clone(), denom) {
        Ok(balance) if json() => Ok(Output::success(balance)),
        Ok(balance) => {
            // Retrieve the key name string to output
            let key_name = key_name.unwrap_or_else(|| {
                let chain_config = chain.config().map_err(|e| {
                    eyre!("`keys balance` command failed due to an error retrieving chain config: {e}")
                })?;
                chain_config.key_name
            });

            Ok(Output::success_msg(format!(
                "balance for key `{key_name}`: {} {}",
                balance.amount, balance.denom
            )))
        }
        Err(e) => Err(e.wrap_err("`keys balance` command failed")),
    }
}

fn get_balances(chain: impl ChainHandle, key_name: Option<String>) -> Result<Output> {
    match chain.query_all_balances(key_name.clone()) {
        Ok(balances) if json() => Ok(Output::success(balances)),
        Ok(balances) => {
            // Retrieve the key name string to output.
            let key_name = key_name.unwrap_or_else(|| {
                let chain_config = chain.config().map_err(|e| {
                    eyre!("`keys balance` command failed due to an error retrieving chain config; {e}")
                })?;
                chain_config.key_name
            });

            let mut pretty_output = format!("Balances for key `{key_name}`:");

            for balance in balances {
                write!(pretty_output, "\n\t{} {}", balance.amount, balance.denom)
                    .map_err(|e| eyre!("failed to write balance output"));
            }

            Ok(Output::success_msg(pretty_output))
        }
        Err(e) => {
            Err(e.wrap_err("`keys balance` command failed due to a problem querying the balance"))
        }
    }
}
