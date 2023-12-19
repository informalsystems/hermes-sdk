use cgp_core::prelude::*;
use futures::lock::{Mutex, MutexGuard};
use ibc_relayer_components::runtime::traits::mutex::ProvideMutex;

pub struct ProvideFuturesMutex;

#[async_trait]
impl<Runtime> ProvideMutex<Runtime> for ProvideFuturesMutex
where
    Runtime: Async,
{
    type Mutex<T: Async> = Mutex<T>;

    type MutexGuard<'a, T: Async> = MutexGuard<'a, T>;

    fn new_mutex<T: Async>(item: T) -> Self::Mutex<T> {
        Mutex::new(item)
    }

    async fn acquire_mutex<'a, T: Async>(mutex: &'a Self::Mutex<T>) -> Self::MutexGuard<'a, T> {
        mutex.lock().await
    }
}
