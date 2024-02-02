use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::address::HasAddressType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::account_prefix::HasAccountPrefix;
use crate::bootstrap::traits::generate_rollup_genesis::RollupGenesisGenerator;
use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;
use crate::types::rollup_genesis_config::{
    AccountsGenesis, BankGenesis, ChainStateGenesis, CoinsToLock, SequencerRegistryGenesis,
    SovereignGenesisConfig, TimeGenesis, TokenGenesis,
};
use crate::types::wallet::SovereignWallet;

pub struct GenerateSovereignGenesis;

impl<Bootstrap, Runtime, ChainDriver> RollupGenesisGenerator<Bootstrap> for GenerateSovereignGenesis
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasRollupGenesisConfigType<RollupGenesisConfig = SovereignGenesisConfig>
        + HasAccountPrefix
        + HasChainDriverType<ChainDriver = ChainDriver>
        + CanRaiseError<bech32::Error>,
    ChainDriver: HasAddressType,
{
    async fn generate_rollup_genesis(
        bootstrap: &Bootstrap,
        sequencer_da_address: &ChainDriver::Address,
    ) -> Result<Bootstrap::RollupGenesisConfig, Bootstrap::Error> {
        let account_prefix = bootstrap.account_prefix();

        let sequencer_wallet = SovereignWallet::generate("sequencer", account_prefix)
            .map_err(Bootstrap::raise_error)?;

        let relayer_wallet =
            SovereignWallet::generate("relayer", account_prefix).map_err(Bootstrap::raise_error)?;

        let user_a_wallet =
            SovereignWallet::generate("user-a", account_prefix).map_err(Bootstrap::raise_error)?;

        let user_b_wallet =
            SovereignWallet::generate("user-b", account_prefix).map_err(Bootstrap::raise_error)?;

        let rollup_genesis = SovereignGenesisConfig {
            accounts: AccountsGenesis { pub_keys: vec![] },
            bank: BankGenesis {
                tokens: vec![TokenGenesis {
                    token_name: "coin".to_owned(),
                    address_and_balances: vec![
                        (sequencer_wallet.address.clone(), 1_000_000_000_000),
                        (relayer_wallet.address.clone(), 1_000_000_000_000),
                        (user_a_wallet.address.clone(), 1_000_000_000_000),
                        (user_b_wallet.address.clone(), 1_000_000_000_000),
                    ],
                    authorized_minters: vec![],
                    salt: 0,
                }],
            },
            chain_state: ChainStateGenesis {
                current_time: TimeGenesis { secs: 0, nanos: 0 },
                gas_price_blocks_depth: 0,
                gas_price_maximum_elasticity: 0,
                initial_gas_price: vec![0],
                minimum_gas_price: vec![0],
            },
            sequencer_registry: SequencerRegistryGenesis {
                seq_rollup_address: sequencer_wallet.address.clone(),
                seq_da_address: sequencer_da_address.to_string(),
                coins_to_lock: CoinsToLock {
                    amount: 0,
                    token_address: "".to_owned(),
                },
                is_preferred_sequencer: true,
            },
        };

        Ok(rollup_genesis)
    }
}
