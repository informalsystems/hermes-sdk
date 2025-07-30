use alloc::collections::BTreeMap;
use core::marker::PhantomData;

use hermes_chain_type_components::traits::HasDenomType;
use hermes_cosmos_test_components::bootstrap::traits::{
    DenomForStaking, DenomForTransfer, HasChainGenesisConfigType, HasGenesisDenom,
    HasWalletConfigType, WalletConfigGenerator, WalletConfigGeneratorComponent,
};
use hermes_cosmos_test_components::bootstrap::types::CosmosWalletConfig;
use hermes_cosmos_test_components::chain::types::{Amount, Denom};
use hermes_prelude::*;
use hermes_test_components::chain_driver::traits::HasChainType;

#[cgp_new_provider(WalletConfigGeneratorComponent)]
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

        let relayer1 = CosmosWalletConfig {
            wallet_id: "relayer1".to_owned(),
            genesis_balances: vec![
                Amount::new(1_000_000_000_000_000_000, denom_for_staking.clone()),
                Amount::new(1_000_000_000_000_000_000, denom_for_transfer.clone()),
            ],
            validator_staked_amount: None,
        };

        let relayer2 = CosmosWalletConfig {
            wallet_id: "relayer2".to_owned(),
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
            ("relayer1".into(), relayer1),
            ("relayer2".into(), relayer2),
        ]))
    }
}
