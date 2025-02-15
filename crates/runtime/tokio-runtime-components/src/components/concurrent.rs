use cgp::prelude::*;
use hermes_async_runtime_components::task::impls::concurrent::RunConcurrentTasks;
pub use hermes_runtime_components::traits::channel::{
    ChannelCreatorComponent, ChannelTypeComponent, ChannelUserComponent, ReceiverStreamerComponent,
    SenderClonerComponent,
};
pub use hermes_runtime_components::traits::channel_once::{
    ChannelOnceCreatorComponent, ChannelOnceTypeComponent, ChannelOnceUserComponent,
};
pub use hermes_runtime_components::traits::fs::copy_file::FileCopierComponent;
pub use hermes_runtime_components::traits::fs::create_dir::DirCreatorComponent;
pub use hermes_runtime_components::traits::fs::file_path::FilePathTypeComponent;
pub use hermes_runtime_components::traits::fs::read_file::FileAsStringReaderComponent;
pub use hermes_runtime_components::traits::fs::write_file::StringToFileWriterComponent;
pub use hermes_runtime_components::traits::mutex::MutexComponent;
pub use hermes_runtime_components::traits::os::child_process::{
    ChildProcessStarterComponent, ChildProcessTypeComponent, ChildProcessWaiterComponent,
};
pub use hermes_runtime_components::traits::os::exec_command::{
    CommandExecutorComponent, CommandWithEnvsExecutorComponent,
};
pub use hermes_runtime_components::traits::os::reserve_port::TcpPortReserverComponent;
pub use hermes_runtime_components::traits::random::RandomGeneratorComponent;
pub use hermes_runtime_components::traits::sleep::SleeperComponent;
pub use hermes_runtime_components::traits::spawn::TaskSpawnerComponent;
pub use hermes_runtime_components::traits::stream::{StreamMapperComponent, StreamTypeComponent};
pub use hermes_runtime_components::traits::subscription::SubscriptionComponent;
pub use hermes_runtime_components::traits::task::ConcurrentTaskRunnerComponent;
pub use hermes_runtime_components::traits::time::TimeComponent;

use crate::components::parallel::TokioParallelRuntimeComponents;

cgp_preset! {
    TokioConcurrentRuntimeComponents {
        ConcurrentTaskRunnerComponent: RunConcurrentTasks,
        [
            SleeperComponent,
            TimeComponent,
            MutexComponent,
            StreamTypeComponent,
            StreamMapperComponent,
            SubscriptionComponent,
            TaskSpawnerComponent,
            ChannelTypeComponent,
            ChannelCreatorComponent,
            ChannelUserComponent,
            ChannelOnceTypeComponent,
            ChannelOnceCreatorComponent,
            ChannelOnceUserComponent,
            ReceiverStreamerComponent,
            SenderClonerComponent,
            FilePathTypeComponent,
            ChildProcessTypeComponent,
            ChildProcessStarterComponent,
            ChildProcessWaiterComponent,
            FileAsStringReaderComponent,
            DirCreatorComponent,
            FileCopierComponent,
            CommandExecutorComponent,
            CommandWithEnvsExecutorComponent,
            StringToFileWriterComponent,
            TcpPortReserverComponent,
            RandomGeneratorComponent,
        ]:
            TokioParallelRuntimeComponents::Provider,
    }
}
