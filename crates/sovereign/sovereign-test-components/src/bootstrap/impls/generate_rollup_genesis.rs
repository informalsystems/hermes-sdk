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
    AccountGenesis, AccountsGenesis, BankGenesis, ChainStateGenesis, ProverIncentivesGenesis,
    SequencerRegistryGenesis, SovereignGenesisConfig, TimeGenesis, TokenGenesis,
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
        _bootstrap: &Bootstrap,
        sequencer_da_address: &Chain::Address,
        rollup_wallets: &BTreeMap<String, Rollup::Wallet>,
    ) -> Result<SovereignGenesisConfig, Bootstrap::Error> {
        let sequencer_wallet = rollup_wallets
            .get("sequencer")
            .ok_or_else(|| Bootstrap::raise_error("expect sequencer wallet to be present"))?;

        let prover_wallet = rollup_wallets
            .get("prover")
            .ok_or_else(|| Bootstrap::raise_error("expect prover wallet to be present"))?;

        let address_and_balances = rollup_wallets
            .values()
            .map(|wallet| (wallet.address.address.clone(), 1_000_000_000_000))
            .collect::<Vec<_>>();

        // The token address is derived based on the code `get_genesis_token_address` at
        // <https://github.com/Sovereign-Labs/sovereign-sdk/blob/c9f56b479c6ea17893e282099fcb8ab804c2feb1/module-system/module-implementations/sov-bank/src/utils.rs#L21>.
        // At the moment of writing, the sender (deployer) address is all zeroes.
        //
        // NOTE: The gas token address is _hardcoded_ as a constant in the rollup starter.
        // So if we use a different address, the rollup bootstrapping would fail.
        let staking_token_address =
            encode_token_address("stake", &[0; 32], 0, "token_").map_err(Bootstrap::raise_error)?;

        let transfer_token_address =
            encode_token_address("coin", &[0; 32], 0, "token_").map_err(Bootstrap::raise_error)?;

        let accounts = rollup_wallets
            .values()
            .map(|wallet| AccountGenesis {
                credential_id: wallet.credential_id.clone(),
                address: wallet.address.address.clone(),
            })
            .collect();

        let rollup_genesis = SovereignGenesisConfig {
            accounts: AccountsGenesis { accounts },
            bank: BankGenesis {
                gas_token_config: TokenGenesis {
                    token_name: "stake".to_owned(),
                    token_id: staking_token_address.address.clone(),
                    address_and_balances: address_and_balances.clone(),
                    authorized_minters: vec![],
                    salt: 0,
                },
                tokens: vec![TokenGenesis {
                    token_name: "coin".to_owned(),
                    token_id: transfer_token_address.address.clone(),
                    address_and_balances,
                    authorized_minters: vec![],
                    salt: 0,
                }],
            },
            chain_state: ChainStateGenesis {
                current_time: TimeGenesis { secs: 0, nanos: 0 },
                genesis_da_height: 0,
                inner_code_commitment: [0; 8],
                outer_code_commitment: [0; 32],
            },
            sequencer_registry: SequencerRegistryGenesis {
                seq_rollup_address: sequencer_wallet.address.address.clone(),
                seq_da_address: sequencer_da_address.to_string(),
                minimum_bond: 10000,
                is_preferred_sequencer: true,
            },
            prover_incentives: ProverIncentivesGenesis {
                proving_penalty: 10,
                minimum_bond: 10,
                initial_provers: vec![(prover_wallet.address.address.clone(), 10)],
            },
            staking_token_address,
            transfer_token_address,
        };

        Ok(rollup_genesis)
    }
}
