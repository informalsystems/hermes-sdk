use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;

#[cgp_component {
  name: ConfigPathGetterComponent,
  provider: ConfigPathGetter,
  context: App,
}]
pub trait HasConfigPath: HasRuntime<Runtime: HasFilePathType> {
    fn config_path(&self) -> &FilePathOf<Self::Runtime>;
}
