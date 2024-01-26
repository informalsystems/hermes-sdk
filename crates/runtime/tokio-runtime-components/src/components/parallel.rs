use cgp_core::prelude::*;
use hermes_async_runtime_components::channel::impls::ProvideUnboundedChannelType;
use hermes_async_runtime_components::channel_once::impls::ProvideOneShotChannelType;
use hermes_async_runtime_components::mutex::impls::mutex::ProvideFuturesMutex;
use hermes_async_runtime_components::stream::impls::boxed::ProvideBoxedStreamType;
use hermes_async_runtime_components::stream::impls::map::BoxedStreamMapper;
use hermes_async_runtime_components::subscription::impls::subscription::ProvideBoxedSubscription;
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
use hermes_test_components::runtime::impls::exec_command::ExecCommandWithNoEnv;
use hermes_test_components::runtime::traits::child_process::ChildProcessStarterComponent;
use hermes_test_components::runtime::traits::copy_file::FileCopierComponent;
use hermes_test_components::runtime::traits::create_dir::DirCreatorComponent;
use hermes_test_components::runtime::traits::exec_command::{
    CommandExecutorComponent, CommandWithEnvsExecutorComponent,
};
use hermes_test_components::runtime::traits::read_file::FileAsStringReaderComponent;
use hermes_test_components::runtime::traits::reserve_port::TcpPortReserverComponent;
use hermes_test_components::runtime::traits::types::child_process::ChildProcessTypeComponent;
use hermes_test_components::runtime::traits::types::file_path::FilePathTypeComponent;
use hermes_test_components::runtime::traits::write_file::StringToFileWriterComponent;

use crate::impls::child_process::StartTokioChildProcess;
use crate::impls::copy_file::TokioCopyFile;
use crate::impls::create_dir::TokioCreateDir;
use crate::impls::exec_command::TokioExecCommand;
use crate::impls::parallel_task::TokioRunParallelTasks;
use crate::impls::read_file::TokioReadFileAsString;
use crate::impls::reserve_port::TokioReserveTcpPort;
use crate::impls::sleep::TokioSleep;
use crate::impls::spawn::TokioSpawnTask;
use crate::impls::time::ProvideStdTime;
use crate::impls::types::child_process::ProvideTokioChildProcessType;
use crate::impls::types::file_path::ProvideStdPathType;
use crate::impls::write_file::TokioWriteStringToFile;

pub struct TokioParallelRuntimeComponents;

delegate_components! {
    #[mark_component(IsTokioParallelRuntimeComponent)]
    #[mark_delegate(DelegatesToTokioParallelRuntimeComponents)]
    TokioParallelRuntimeComponents {
        SleeperComponent: TokioSleep,
        TimeComponent: ProvideStdTime,
        MutexComponent: ProvideFuturesMutex,
        StreamTypeComponent: ProvideBoxedStreamType,
        StreamMapperComponent: BoxedStreamMapper,
        SubscriptionComponent: ProvideBoxedSubscription,
        ConcurrentTaskRunnerComponent: TokioRunParallelTasks,
        TaskSpawnerComponent: TokioSpawnTask,
        [
            ChannelTypeComponent,
            ChannelCreatorComponent,
            ChannelUserComponent,
            ReceiverStreamerComponent,
            SenderClonerComponent,
        ]: ProvideUnboundedChannelType,
        [
            ChannelOnceTypeComponent,
            ChannelOnceCreatorComponent,
            ChannelOnceUserComponent,
        ]:
            ProvideOneShotChannelType,
        FilePathTypeComponent: ProvideStdPathType,
        ChildProcessTypeComponent: ProvideTokioChildProcessType,
        ChildProcessStarterComponent: StartTokioChildProcess,
        FileAsStringReaderComponent: TokioReadFileAsString,
        DirCreatorComponent: TokioCreateDir,
        FileCopierComponent: TokioCopyFile,
        CommandWithEnvsExecutorComponent: TokioExecCommand,
        CommandExecutorComponent: ExecCommandWithNoEnv,
        StringToFileWriterComponent: TokioWriteStringToFile,
        TcpPortReserverComponent: TokioReserveTcpPort,
    }
}
