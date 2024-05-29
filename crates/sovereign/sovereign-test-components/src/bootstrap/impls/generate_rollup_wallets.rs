use alloc::collections::BTreeMap;

use cgp_core::CanRaiseError;
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::HasAccountPrefix;
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::HasRollupType;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;

use crate::bootstrap::traits::generate_rollup_wallets::RollupWalletGenerator;
use crate::types::wallet::SovereignWallet;

pub struct GenerateSovereignRollupWallets;

impl<Bootstrap, Rollup> RollupWalletGenerator<Bootstrap> for GenerateSovereignRollupWallets
where
    Bootstrap: HasRollupType<Rollup = Rollup> + HasAccountPrefix + CanRaiseError<bech32::Error>,
    Rollup: HasWalletType<Wallet = SovereignWallet>,
{
    async fn generate_rollup_wallets(
        bootstrap: &Bootstrap,
    ) -> Result<BTreeMap<String, SovereignWallet>, Bootstrap::Error> {
        let account_prefix = bootstrap.account_prefix();
        let wallet_ids = ["sequencer", "prover", "relayer", "user-a", "user-b"];

        let wallets = wallet_ids
            .iter()
            .map(|wallet_id| {
                let wallet = SovereignWallet::generate(wallet_id, account_prefix)?;
                Ok((wallet_id.to_string(), wallet))
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(Bootstrap::raise_error)?;

        Ok(BTreeMap::from_iter(wallets))
    }
}
