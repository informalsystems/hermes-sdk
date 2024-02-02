use cgp_core::CanRaiseError;
use hermes_test_components::chain_driver::traits::types::wallet::HasWalletType;

use crate::bootstrap::traits::account_prefix::HasAccountPrefix;
use crate::bootstrap::traits::generate_rollup_wallets::RollupWalletGenerator;
use crate::bootstrap::traits::types::rollup_driver::HasRollupDriverType;
use crate::types::wallet::SovereignWallet;

pub struct GenerateSovereignRollupWallets;

impl<Bootstrap, RollupDriver> RollupWalletGenerator<Bootstrap> for GenerateSovereignRollupWallets
where
    Bootstrap: HasRollupDriverType<RollupDriver = RollupDriver>
        + HasAccountPrefix
        + CanRaiseError<bech32::Error>,
    RollupDriver: HasWalletType<Wallet = SovereignWallet>,
{
    async fn generate_rollup_wallets(
        bootstrap: &Bootstrap,
    ) -> Result<Vec<SovereignWallet>, Bootstrap::Error> {
        let account_prefix = bootstrap.account_prefix();
        let wallet_ids = ["sequencer", "relayer", "user-a", "user-b"];

        let wallets = wallet_ids
            .iter()
            .map(|wallet_id| SovereignWallet::generate(wallet_id, account_prefix))
            .collect::<Result<Vec<_>, _>>()
            .map_err(Bootstrap::raise_error)?;

        Ok(wallets)
    }
}
