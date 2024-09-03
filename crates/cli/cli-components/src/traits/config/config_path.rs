use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;

#[derive_component(ConfigPathGetterComponent, ConfigPathGetter<App>)]
pub trait HasConfigPath: HasRuntime<Runtime: HasFilePathType> {
    fn config_path(&self) -> &FilePathOf<Self::Runtime>;
}
