use core::ops::DerefMut;

use cgp::prelude::*;

#[cgp_component {
  name: MutexComponent,
  provider: ProvideMutex,
  context: Runtime,
}]
#[async_trait]
pub trait HasMutex: Async {
    type Mutex<T: Async>: Async;

    type MutexGuard<'a, T: Async>: 'a + Send + Sync + DerefMut<Target = T>;

    fn new_mutex<T: Async>(item: T) -> Self::Mutex<T>;

    async fn acquire_mutex<'a, T: Async>(mutex: &'a Self::Mutex<T>) -> Self::MutexGuard<'a, T>;
}

pub type MutexOf<Runtime, T> = <Runtime as HasMutex>::Mutex<T>;

pub type MutexGuardOf<'a, Runtime, T> = <Runtime as HasMutex>::MutexGuard<'a, T>;
