use alloc::collections::BTreeMap;

use cgp_core::CanRaiseError;
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::HasAccountPrefix;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::HasRollupType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::generate_rollup_genesis::RollupGenesisGenerator;
use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;
use crate::types::rollup_genesis_config::{
    AccountsGenesis, BankGenesis, ChainStateGenesis, CoinsToLock, SequencerRegistryGenesis,
    SovereignGenesisConfig, TimeGenesis, TokenGenesis,
};
use crate::types::wallet::{encode_token_address, SovereignWallet};

pub struct GenerateSovereignGenesis;

impl<Bootstrap, Runtime, Chain, Rollup> RollupGenesisGenerator<Bootstrap>
    for GenerateSovereignGenesis
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasRollupGenesisConfigType<RollupGenesisConfig = SovereignGenesisConfig>
        + HasAccountPrefix
        + HasChainType<Chain = Chain>
        + HasRollupType<Rollup = Rollup>
        + CanRaiseError<bech32::Error>
        + CanRaiseError<&'static str>,
    Chain: HasAddressType,
    Rollup: HasWalletType<Wallet = SovereignWallet>,
{
    async fn generate_rollup_genesis(
        bootstrap: &Bootstrap,
        sequencer_da_address: &Chain::Address,
        rollup_wallets: &BTreeMap<String, Rollup::Wallet>,
    ) -> Result<SovereignGenesisConfig, Bootstrap::Error> {
        let sequencer_wallet = rollup_wallets
            .get("sequencer")
            .ok_or_else(|| Bootstrap::raise_error("expect sequencer wallet to be present"))?;

        let address_and_balances = rollup_wallets
            .values()
            .map(|wallet| (wallet.address.address.clone(), 1_000_000_000_000))
            .collect::<Vec<_>>();

        // The token address is derived based on the code `get_genesis_token_address` at
        // <https://github.com/Sovereign-Labs/sovereign-sdk/blob/c9f56b479c6ea17893e282099fcb8ab804c2feb1/module-system/module-implementations/sov-bank/src/utils.rs#L21>.
        // At the moment of writing, the sender (deployer) address is all zeroes.
        let staking_token_address =
            encode_token_address("stake", &[0; 32], 0, bootstrap.account_prefix())
                .map_err(Bootstrap::raise_error)?;

        let transfer_token_address =
            encode_token_address("coin", &[0; 32], 0, bootstrap.account_prefix())
                .map_err(Bootstrap::raise_error)?;

        let rollup_genesis = SovereignGenesisConfig {
            accounts: AccountsGenesis { pub_keys: vec![] },
            bank: BankGenesis {
                tokens: vec![
                    TokenGenesis {
                        token_name: "stake".to_owned(),
                        token_address: staking_token_address.address.clone(),
                        address_and_balances: address_and_balances.clone(),
                        authorized_minters: vec![],
                        salt: 0,
                    },
                    TokenGenesis {
                        token_name: "coin".to_owned(),
                        token_address: transfer_token_address.address.clone(),
                        address_and_balances,
                        authorized_minters: vec![],
                        salt: 0,
                    },
                ],
            },
            chain_state: ChainStateGenesis {
                current_time: TimeGenesis { secs: 0, nanos: 0 },
                gas_price_blocks_depth: 10,
                gas_price_maximum_elasticity: 5,
                initial_gas_price: (0, 0),
                minimum_gas_price: (0, 0),
            },
            sequencer_registry: SequencerRegistryGenesis {
                seq_rollup_address: sequencer_wallet.address.address.clone(),
                seq_da_address: sequencer_da_address.to_string(),
                coins_to_lock: CoinsToLock {
                    amount: 1,
                    token_address: staking_token_address.address.clone(),
                },
                is_preferred_sequencer: true,
            },
            staking_token_address,
            transfer_token_address,
        };

        Ok(rollup_genesis)
    }
}
