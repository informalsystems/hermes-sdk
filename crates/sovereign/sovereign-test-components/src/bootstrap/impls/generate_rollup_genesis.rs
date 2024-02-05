use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::address::HasAddressType;
use hermes_test_components::chain_driver::traits::types::wallet::HasWalletType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::account_prefix::HasAccountPrefix;
use crate::bootstrap::traits::generate_rollup_genesis::RollupGenesisGenerator;
use crate::bootstrap::traits::types::rollup_driver::HasRollupDriverType;
use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;
use crate::types::rollup_genesis_config::{
    AccountsGenesis, BankGenesis, ChainStateGenesis, CoinsToLock, SequencerRegistryGenesis,
    SovereignGenesisConfig, TimeGenesis, TokenGenesis,
};
use crate::types::wallet::SovereignWallet;

pub struct GenerateSovereignGenesis;

impl<Bootstrap, Runtime, ChainDriver, RollupDriver> RollupGenesisGenerator<Bootstrap>
    for GenerateSovereignGenesis
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasRollupGenesisConfigType<RollupGenesisConfig = SovereignGenesisConfig>
        + HasAccountPrefix
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasRollupDriverType<RollupDriver = RollupDriver>
        + CanRaiseError<bech32::Error>
        + CanRaiseError<&'static str>,
    ChainDriver: HasAddressType,
    RollupDriver: HasWalletType<Wallet = SovereignWallet>,
{
    async fn generate_rollup_genesis(
        _bootstrap: &Bootstrap,
        sequencer_da_address: &ChainDriver::Address,
        rollup_wallets: &[RollupDriver::Wallet],
    ) -> Result<Bootstrap::RollupGenesisConfig, Bootstrap::Error> {
        let sequencer_wallet = rollup_wallets
            .iter()
            .find(|wallet| &wallet.wallet_id == "sequencer")
            .ok_or_else(|| Bootstrap::raise_error("expect sequencer wallet to be present"))?;

        let address_and_balances = rollup_wallets
            .iter()
            .map(|wallet| (wallet.address.clone(), 1_000_000_000_000))
            .collect::<Vec<_>>();

        let rollup_genesis = SovereignGenesisConfig {
            accounts: AccountsGenesis { pub_keys: vec![] },
            bank: BankGenesis {
                tokens: vec![
                    TokenGenesis {
                        token_name: "stake".to_owned(),
                        address_and_balances: address_and_balances.clone(),
                        authorized_minters: vec![],
                        salt: 0,
                    },
                    TokenGenesis {
                        token_name: "coin".to_owned(),
                        address_and_balances,
                        authorized_minters: vec![],
                        salt: 0,
                    },
                ],
            },
            chain_state: ChainStateGenesis {
                initial_slot_height: 0,
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
                    token_address: sequencer_wallet.address.clone(),
                },
                is_preferred_sequencer: true,
            },
        };

        Ok(rollup_genesis)
    }
}
