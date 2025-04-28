use hermes_core::runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};
use hermes_prelude::*;

#[cgp_component {
  provider: ConfigPathGetter,
  context: App,
}]
pub trait HasConfigPath: HasRuntime<Runtime: HasFilePathType> {
    fn config_path(&self) -> &FilePathOf<Self::Runtime>;
}
