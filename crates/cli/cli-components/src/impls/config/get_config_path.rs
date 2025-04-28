use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_core::runtime_components::traits::{HasFilePathType, HasRuntime};

use crate::traits::{ConfigPathGetter, ConfigPathGetterComponent};

pub struct GetConfigField<Key>(pub PhantomData<Key>);

pub type GetDefaultConfigField = GetConfigField<symbol!("config_path")>;

#[cgp_provider(ConfigPathGetterComponent)]
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
