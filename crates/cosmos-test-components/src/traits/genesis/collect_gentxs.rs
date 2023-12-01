use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

use crate::traits::types::file_path::HasFilePathType;

#[derive_component(GentxsCollectorComponent, GentxsCollector<Bootstrap>)]
#[async_trait]
pub trait CanCollectGentxs: HasFilePathType + HasChainIdType + HasErrorType {
    async fn collect_gentxs(&self, chain_home_dir: &Self::FilePath) -> Result<(), Self::Error>;
}
