use alloc::collections::BTreeMap;
use core::marker::PhantomData;

use hermes_core::chain_type_components::traits::HasDenomType;
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_prelude::*;

use crate::bootstrap::traits::{
    DenomForStaking, DenomForTransfer, HasChainGenesisConfigType, HasGenesisDenom,
    HasWalletConfigType, WalletConfigGenerator, WalletConfigGeneratorComponent,
};
use crate::bootstrap::types::CosmosWalletConfig;
use crate::chain::types::{Amount, Denom};

/**
   Generator for standard sets of wallets for testing. Consists of one validator wallet,
   two user wallets, and one relayer wallet.

   If a bootstrap context requires custom generation of test wallets, one can implement
   a custom `WalletConfigGenerator` that works similar to this component.
*/
pub struct GenerateStandardWalletConfig;

#[cgp_provider(WalletConfigGeneratorComponent)]
impl<Bootstrap, Chain> WalletConfigGenerator<Bootstrap> for GenerateStandardWalletConfig
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
        // TODO: allow for randomization of denoms and amount

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

        let relayer_2 = CosmosWalletConfig {
            wallet_id: "relayer-2".to_owned(),
            genesis_balances: vec![
                Amount::new(1_000_000_000_000_000_000, denom_for_staking.clone()),
                Amount::new(1_000_000_000_000_000_000, denom_for_transfer.clone()),
            ],
            validator_staked_amount: None,
        };

        Ok(BTreeMap::from([
            ("validator".into(), validator),
            ("user1".into(), user1),
            ("user2".into(), user2),
            ("relayer".into(), relayer),
            ("relayer-2".into(), relayer_2),
        ]))
    }
}
