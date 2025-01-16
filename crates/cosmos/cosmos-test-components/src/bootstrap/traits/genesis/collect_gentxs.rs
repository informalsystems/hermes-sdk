use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;

#[cgp_component {
  provider: GenesisTransactionsCollector,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanCollectGenesisTransactions: HasRuntime + HasAsyncErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn collect_genesis_transactions(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
    ) -> Result<(), Self::Error>;
}
