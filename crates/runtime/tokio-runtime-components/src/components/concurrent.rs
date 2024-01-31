use cgp_core::prelude::*;
use hermes_async_runtime_components::task::impls::concurrent::RunConcurrentTasks;
use hermes_relayer_components::runtime::traits::mutex::MutexComponent;
use hermes_relayer_components::runtime::traits::sleep::SleeperComponent;
use hermes_relayer_components::runtime::traits::stream::{
    StreamMapperComponent, StreamTypeComponent,
};
use hermes_relayer_components::runtime::traits::subscription::SubscriptionComponent;
use hermes_relayer_components::runtime::traits::task::ConcurrentTaskRunnerComponent;
use hermes_relayer_components::runtime::traits::time::TimeComponent;
use hermes_relayer_components_extra::runtime::traits::channel::{
    ChannelCreatorComponent, ChannelTypeComponent, ChannelUserComponent, ReceiverStreamerComponent,
    SenderClonerComponent,
};
use hermes_relayer_components_extra::runtime::traits::channel_once::{
    ChannelOnceCreatorComponent, ChannelOnceTypeComponent, ChannelOnceUserComponent,
};
use hermes_relayer_components_extra::runtime::traits::spawn::TaskSpawnerComponent;
use hermes_test_components::runtime::traits::child_process::ChildProcessStarterComponent;
use hermes_test_components::runtime::traits::copy_file::FileCopierComponent;
use hermes_test_components::runtime::traits::create_dir::DirCreatorComponent;
use hermes_test_components::runtime::traits::exec_command::{
    CommandExecutorComponent, CommandWithEnvsExecutorComponent,
};
use hermes_test_components::runtime::traits::random::RandomGeneratorComponent;
use hermes_test_components::runtime::traits::read_file::FileAsStringReaderComponent;
use hermes_test_components::runtime::traits::reserve_port::TcpPortReserverComponent;
use hermes_test_components::runtime::traits::types::child_process::ChildProcessTypeComponent;
use hermes_test_components::runtime::traits::types::file_path::FilePathTypeComponent;
use hermes_test_components::runtime::traits::write_file::StringToFileWriterComponent;

use crate::components::parallel::TokioParallelRuntimeComponents;

pub struct TokioConcurrentRuntimeComponents;

delegate_components! {
    #[mark_component(IsTokioConcurrentRuntimeComponent)]
    #[mark_delegate(DelegatesToTokioConcurrentRuntimeComponents)]
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
