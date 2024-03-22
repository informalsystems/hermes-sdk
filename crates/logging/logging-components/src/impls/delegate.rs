use core::marker::PhantomData;

use cgp_core::prelude::Async;
use cgp_core::DelegateComponent;

use crate::traits::logger::Logger;

pub struct DelegateLogger<Components>(pub PhantomData<Components>);

impl<Logging, Components, Delegate, Details> Logger<Logging, Details> for DelegateLogger<Components>
where
    Logging: Async,
    Details: Send + Sync,
    Components: DelegateComponent<Details, Delegate = Delegate>,
    Delegate: Logger<Logging, Details>,
{
    async fn log(logging: &Logging, message: &str, details: &Details) {
        Delegate::log(logging, message, details).await
    }
}
