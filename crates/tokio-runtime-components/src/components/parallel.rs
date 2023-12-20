use async_runtime_components::channel::impls::ProvideUnboundedChannelType;
use async_runtime_components::channel_once::impls::ProvideOneShotChannelType;
use async_runtime_components::mutex::impls::mutex::ProvideFuturesMutex;
use async_runtime_components::stream::impls::boxed::ProvideBoxedStreamType;
use async_runtime_components::stream::impls::map::BoxedStreamMapper;
use async_runtime_components::subscription::impls::subscription::ProvideBoxedSubscription;
use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::mutex::MutexComponent;
use ibc_relayer_components::runtime::traits::sleep::SleeperComponent;
use ibc_relayer_components::runtime::traits::stream::{StreamMapperComponent, StreamTypeComponent};
use ibc_relayer_components::runtime::traits::subscription::SubscriptionComponent;
use ibc_relayer_components::runtime::traits::task::ConcurrentTaskRunnerComponent;
use ibc_relayer_components::runtime::traits::time::TimeComponent;
use ibc_relayer_components_extra::runtime::traits::channel::{
    ChannelCreatorComponent, ChannelTypeComponent, ChannelUserComponent, ReceiverStreamerComponent,
    SenderClonerComponent,
};
use ibc_relayer_components_extra::runtime::traits::channel_once::{
    ChannelOnceCreatorComponent, ChannelOnceTypeComponent, ChannelOnceUserComponent,
};
use ibc_relayer_components_extra::runtime::traits::spawn::TaskSpawnerComponent;
use ibc_test_components::runtime::traits::child_process::ChildProcessStarterComponent;
use ibc_test_components::runtime::traits::exec_command::CommandExecutorComponent;
use ibc_test_components::runtime::traits::read_file::FileAsStringReaderComponent;
use ibc_test_components::runtime::traits::reserve_port::TcpPortReserverComponent;
use ibc_test_components::runtime::traits::types::child_process::ChildProcessTypeComponent;
use ibc_test_components::runtime::traits::types::file_path::FilePathTypeComponent;
use ibc_test_components::runtime::traits::write_file::StringToFileWriterComponent;

use crate::impls::child_process::StartTokioChildProcess;
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
        CommandExecutorComponent: TokioExecCommand,
        StringToFileWriterComponent: TokioWriteStringToFile,
        TcpPortReserverComponent: TokioReserveTcpPort,
    }
}
