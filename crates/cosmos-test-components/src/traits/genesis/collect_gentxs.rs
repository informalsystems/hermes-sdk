use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;

use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(GenesisTransactionsCollectorComponent, GenesisTransactionsCollector<Bootstrap>)]
#[async_trait]
pub trait CanCollectGenesisTransactions: HasRuntime + HasChainIdType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn collect_genesis_transactions(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
    ) -> Result<(), Self::Error>;
}
