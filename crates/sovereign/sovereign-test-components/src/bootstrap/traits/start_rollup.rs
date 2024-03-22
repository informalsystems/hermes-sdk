use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;

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
