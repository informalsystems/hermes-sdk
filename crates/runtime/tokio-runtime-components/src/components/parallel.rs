use cgp_core::prelude::*;
use hermes_async_runtime_components::channel::impls::ProvideUnboundedChannelType;
use hermes_async_runtime_components::channel_once::impls::ProvideOneShotChannelType;
use hermes_async_runtime_components::mutex::impls::mutex::ProvideFuturesMutex;
use hermes_async_runtime_components::stream::impls::boxed::ProvideBoxedStreamType;
use hermes_async_runtime_components::stream::impls::map::BoxedStreamMapper;
use hermes_async_runtime_components::subscription::impls::subscription::ProvideBoxedSubscription;
use hermes_runtime_components::impls::os::exec_command::ExecCommandWithNoEnv;
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
pub use hermes_runtime_components::traits::os::child_process::ChildProcessWaiterComponent;
pub use hermes_runtime_components::traits::os::child_process::{
    ChildProcessStarterComponent, ChildProcessTypeComponent,
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

use crate::impls::fs::copy_file::TokioCopyFile;
use crate::impls::fs::create_dir::TokioCreateDir;
use crate::impls::fs::file_path::ProvideStdPathType;
use crate::impls::fs::read_file::TokioReadFileAsString;
use crate::impls::fs::write_file::TokioWriteStringToFile;
use crate::impls::os::child_process::{
    ProvideTokioChildProcessType, StartTokioChildProcess, WaitChildProcess,
};
use crate::impls::os::exec_command::TokioExecCommand;
use crate::impls::os::reserve_port::TokioReserveTcpPort;
use crate::impls::parallel_task::TokioRunParallelTasks;
use crate::impls::random::ThreadRandomGenerator;
use crate::impls::sleep::TokioSleep;
use crate::impls::spawn::TokioSpawnTask;
use crate::impls::time::ProvideStdTime;

define_components! {
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
        ChildProcessWaiterComponent: WaitChildProcess,
        FileAsStringReaderComponent: TokioReadFileAsString,
        DirCreatorComponent: TokioCreateDir,
        FileCopierComponent: TokioCopyFile,
        CommandWithEnvsExecutorComponent: TokioExecCommand,
        CommandExecutorComponent: ExecCommandWithNoEnv,
        StringToFileWriterComponent: TokioWriteStringToFile,
        TcpPortReserverComponent: TokioReserveTcpPort,
        RandomGeneratorComponent: ThreadRandomGenerator,
    }
}
