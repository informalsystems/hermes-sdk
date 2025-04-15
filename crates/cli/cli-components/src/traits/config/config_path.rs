use cgp::prelude::*;
use hermes_runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};

#[cgp_component {
  provider: ConfigPathGetter,
  context: App,
}]
pub trait HasConfigPath: HasRuntime<Runtime: HasFilePathType> {
    fn config_path(&self) -> &FilePathOf<Self::Runtime>;
}
