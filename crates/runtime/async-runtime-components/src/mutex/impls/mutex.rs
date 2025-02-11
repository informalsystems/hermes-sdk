use cgp::prelude::*;
use futures_util::lock::{Mutex, MutexGuard};
use hermes_runtime_components::traits::mutex::{MutexComponent, ProvideMutex};

pub struct ProvideFuturesMutex;

#[cgp_provider(MutexComponent)]
impl<Runtime> ProvideMutex<Runtime> for ProvideFuturesMutex
where
    Runtime: Async,
{
    type Mutex<T: Async> = Mutex<T>;

    type MutexGuard<'a, T: Async> = MutexGuard<'a, T>;

    fn new_mutex<T: Async>(item: T) -> Self::Mutex<T> {
        Mutex::new(item)
    }

    async fn acquire_mutex<T: Async>(mutex: &Self::Mutex<T>) -> Self::MutexGuard<'_, T> {
        mutex.lock().await
    }
}
