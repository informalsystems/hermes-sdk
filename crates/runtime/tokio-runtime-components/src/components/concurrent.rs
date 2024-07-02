use cgp_core::prelude::*;
use hermes_async_runtime_components::task::impls::concurrent::RunConcurrentTasks;
use hermes_runtime_components::traits::channel::{
    ChannelCreatorComponent, ChannelTypeComponent, ChannelUserComponent, ReceiverStreamerComponent,
    SenderClonerComponent,
};
use hermes_runtime_components::traits::channel_once::{
    ChannelOnceCreatorComponent, ChannelOnceTypeComponent, ChannelOnceUserComponent,
};
use hermes_runtime_components::traits::fs::copy_file::FileCopierComponent;
use hermes_runtime_components::traits::fs::create_dir::DirCreatorComponent;
use hermes_runtime_components::traits::fs::file_path::FilePathTypeComponent;
use hermes_runtime_components::traits::fs::read_file::FileAsStringReaderComponent;
use hermes_runtime_components::traits::fs::write_file::StringToFileWriterComponent;
use hermes_runtime_components::traits::mutex::MutexComponent;
use hermes_runtime_components::traits::os::child_process::{
    ChildProcessStarterComponent, ChildProcessTypeComponent,
};
use hermes_runtime_components::traits::os::exec_command::{
    CommandExecutorComponent, CommandWithEnvsExecutorComponent,
};
use hermes_runtime_components::traits::os::reserve_port::TcpPortReserverComponent;
use hermes_runtime_components::traits::random::RandomGeneratorComponent;
use hermes_runtime_components::traits::sleep::SleeperComponent;
use hermes_runtime_components::traits::spawn::TaskSpawnerComponent;
use hermes_runtime_components::traits::stream::{StreamMapperComponent, StreamTypeComponent};
use hermes_runtime_components::traits::subscription::SubscriptionComponent;
use hermes_runtime_components::traits::task::ConcurrentTaskRunnerComponent;
use hermes_runtime_components::traits::time::TimeComponent;

use crate::components::parallel::TokioParallelRuntimeComponents;

define_components! {
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
            FileAsStringReaderComponent,
            DirCreatorComponent,
            FileCopierComponent,
            CommandExecutorComponent,
            CommandWithEnvsExecutorComponent,
            StringToFileWriterComponent,
            TcpPortReserverComponent,
            RandomGeneratorComponent,
        ]:
            TokioParallelRuntimeComponents,
    }
}
