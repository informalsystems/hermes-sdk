use cgp_core::prelude::*;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

#[derive_component(GenesisTransactionsCollectorComponent, GenesisTransactionsCollector<Bootstrap>)]
#[async_trait]
pub trait CanCollectGenesisTransactions: HasRuntime + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn collect_genesis_transactions(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
    ) -> Result<(), Self::Error>;
}
