use cgp_core::prelude::*;

#[async_trait]
pub trait CanGenerateWalletId: Async {
    async fn generate_wallet_id(&self, wallet_id_prefix: &str) -> String;
}
