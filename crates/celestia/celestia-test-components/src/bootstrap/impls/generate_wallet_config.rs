use alloc::collections::BTreeMap;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::fields::denom::{
    DenomForStaking, DenomForTransfer, HasGenesisDenom,
};
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::{
    WalletConfigGenerator, WalletConfigGeneratorComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::wallet_config::HasWalletConfigType;
use hermes_cosmos_test_components::bootstrap::types::wallet_config::CosmosWalletConfig;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_cosmos_test_components::chain::types::denom::Denom;
use hermes_test_components::chain::traits::types::denom::HasDenomType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

pub struct GenerateCelestiaWalletConfig;

#[cgp_provider(WalletConfigGeneratorComponent)]
impl<Bootstrap, Chain> WalletConfigGenerator<Bootstrap> for GenerateCelestiaWalletConfig
where
    Bootstrap: HasWalletConfigType<WalletConfig = CosmosWalletConfig>
        + HasChainType<Chain = Chain>
        + HasChainGenesisConfigType
        + HasAsyncErrorType
        + HasGenesisDenom<DenomForStaking>
        + HasGenesisDenom<DenomForTransfer>,
    Chain: HasDenomType<Denom = Denom>,
{
    async fn generate_wallet_configs(
        _bootstrap: &Bootstrap,
        genesis_config: &Bootstrap::ChainGenesisConfig,
    ) -> Result<BTreeMap<String, CosmosWalletConfig>, Bootstrap::Error> {
        let denom_for_staking =
            Bootstrap::genesis_denom(genesis_config, PhantomData::<DenomForStaking>);

        let denom_for_transfer =
            Bootstrap::genesis_denom(genesis_config, PhantomData::<DenomForTransfer>);

        let validator = CosmosWalletConfig {
            wallet_id: "validator".to_owned(),
            genesis_balances: vec![
                Amount::new(2_000_000_000_000_000_000, denom_for_staking.clone()),
                Amount::new(1_000_000_000_000_000_000, denom_for_transfer.clone()),
            ],
            validator_staked_amount: Some(Amount::new(
                1_000_000_000_000_000_000,
                denom_for_staking.clone(),
            )),
        };

        let sequencer = CosmosWalletConfig {
            wallet_id: "sequencer".to_owned(),
            genesis_balances: vec![
                Amount::new(2_000_000_000_000_000_000, denom_for_staking.clone()),
                Amount::new(1_000_000_000_000_000_000, denom_for_transfer.clone()),
            ],
            validator_staked_amount: None,
        };

        let user1 = CosmosWalletConfig {
            wallet_id: "user1".to_owned(),
            genesis_balances: vec![
                Amount::new(1_000_000_000_000_000_000, denom_for_staking.clone()),
                Amount::new(1_000_000_000_000_000_000, denom_for_transfer.clone()),
            ],
            validator_staked_amount: None,
        };

        let user2 = CosmosWalletConfig {
            wallet_id: "user2".to_owned(),
            genesis_balances: vec![
                Amount::new(1_000_000_000_000_000_000, denom_for_staking.clone()),
                Amount::new(1_000_000_000_000_000_000, denom_for_transfer.clone()),
            ],
            validator_staked_amount: None,
        };

        let relayer = CosmosWalletConfig {
            wallet_id: "relayer".to_owned(),
            genesis_balances: vec![
                Amount::new(1_000_000_000_000_000_000, denom_for_staking.clone()),
                Amount::new(1_000_000_000_000_000_000, denom_for_transfer.clone()),
            ],
            validator_staked_amount: None,
        };

        Ok(BTreeMap::from([
            ("validator".into(), validator),
            ("sequencer".into(), sequencer),
            ("user1".into(), user1),
            ("user2".into(), user2),
            ("relayer".into(), relayer),
        ]))
    }
}
