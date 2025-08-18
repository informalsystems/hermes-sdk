use hermes_prelude::*;

#[cgp_component {
    provider: TokenCliTransferrer,
    context: ChainDriver,
}]
#[async_trait]
pub trait CanCliTransferToken: HasAsyncErrorType {
    async fn cli_transfer_token(
        &self,
        port_id: &str,
        channel_id: &str,
        sender: &str,
        recipient: &str,
        amount: &str,
        fees: &str,
    ) -> Result<(), Self::Error>;
}
