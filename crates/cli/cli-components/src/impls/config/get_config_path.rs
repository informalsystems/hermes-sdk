use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::traits::config::config_path::ConfigPathGetter;

pub struct GetConfigField<Key>(pub PhantomData<Key>);

pub type GetDefaultConfigField = GetConfigField<symbol!("config_path")>;

impl<App, Key, Runtime> ConfigPathGetter<App> for GetConfigField<Key>
where
    App: HasRuntime<Runtime = Runtime> + HasField<Key, Value = Runtime::FilePath>,
    Runtime: HasFilePathType,
    Key: Async,
{
    fn config_path(app: &App) -> &Runtime::FilePath {
        app.get_field(PhantomData)
    }
}
