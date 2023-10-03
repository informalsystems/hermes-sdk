use cgp_core::traits::Async;
use ibc_relayer_components::runtime::traits::task::Task;

pub trait CanSpawnTask: Async {
    fn spawn_task<T>(&self, task: T)
    where
        T: Task;
}
