use alloc::string::String;

use hashbrown::HashMap;
use hermes_prelude::*;

#[cgp_component {
    provider: TokenCliTransferrer,
    context: ChainDriver,
}]
#[async_trait]
pub trait CanCliTransferToken: HasAsyncErrorType {
    async fn cli_transfer_token(&self, args: HashMap<&str, String>) -> Result<(), Self::Error>;
}
