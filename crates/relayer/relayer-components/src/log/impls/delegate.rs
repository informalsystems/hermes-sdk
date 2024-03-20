use core::marker::PhantomData;

use cgp_core::prelude::Async;
use cgp_core::DelegateComponent;

use crate::log::traits::logger::Logger;

pub struct DelegateLogger<Components>(pub PhantomData<Components>);

impl<Context, Components, Delegate, Details> Logger<Context, Details> for DelegateLogger<Components>
where
    Context: Async,
    Details: Send + Sync,
    Components: DelegateComponent<Details, Delegate = Delegate>,
    Delegate: Logger<Context, Details>,
{
    async fn log(context: &Context, message: &str, details: Details) {
        Delegate::log(context, message, details).await
    }
}
