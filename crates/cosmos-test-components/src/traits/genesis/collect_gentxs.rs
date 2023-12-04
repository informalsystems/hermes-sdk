use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

use crate::traits::types::io::file_path::HasFilePathType;

#[derive_component(GenesisTransactionsCollectorComponent, GenesisTransactionsCollector<Bootstrap>)]
#[async_trait]
pub trait CanCollectGenesisTransactions: HasFilePathType + HasChainIdType + HasErrorType {
    async fn collect_genesis_transactions(
        &self,
        chain_home_dir: &Self::FilePath,
    ) -> Result<(), Self::Error>;
}
