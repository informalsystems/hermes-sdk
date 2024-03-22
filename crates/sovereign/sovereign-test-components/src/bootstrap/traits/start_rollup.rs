use cgp_core::prelude::*;
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcessOf, HasChildProcessType,
};
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

#[derive_component(RollupStarterComponent, RollupStarter<Bootstrap>)]
#[async_trait]
pub trait CanStartRollup: HasRuntimeType + HasErrorType
where
    Self::Runtime: HasChildProcessType + HasFilePathType,
{
    async fn start_rollup(
        &self,
        rollup_home_dir: &FilePathOf<Self::Runtime>,
    ) -> Result<ChildProcessOf<Self::Runtime>, Self::Error>;
}
