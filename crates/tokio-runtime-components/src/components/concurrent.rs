use async_runtime_components::task::impls::concurrent::RunConcurrentTasks;
use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::mutex::MutexComponent;
use ibc_relayer_components::runtime::traits::sleep::SleeperComponent;
use ibc_relayer_components::runtime::traits::stream::{StreamMapperComponent, StreamTypeComponent};
use ibc_relayer_components::runtime::traits::subscription::SubscriptionComponent;
use ibc_relayer_components::runtime::traits::task::ConcurrentTaskRunnerComponent;
use ibc_relayer_components::runtime::traits::time::TimeComponent;
use ibc_relayer_components_extra::runtime::traits::channel::{
    ChannelCreatorComponent, ChannelTypeComponent, ChannelUserComponent, ReceiverStreamerComponent,
};
use ibc_test_components::runtime::traits::child_process::ChildProcessStarterComponent;
use ibc_test_components::runtime::traits::exec_command::CommandExecutorComponent;
use ibc_test_components::runtime::traits::read_file::FileAsStringReaderComponent;
use ibc_test_components::runtime::traits::reserve_port::TcpPortReserverComponent;
use ibc_test_components::runtime::traits::types::child_process::ChildProcessTypeComponent;
use ibc_test_components::runtime::traits::types::file_path::FilePathTypeComponent;
use ibc_test_components::runtime::traits::write_file::StringToFileWriterComponent;

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
            ChannelTypeComponent,
            ChannelCreatorComponent,
            ChannelUserComponent,
            ReceiverStreamerComponent,
            FilePathTypeComponent,
            ChildProcessTypeComponent,
            ChildProcessStarterComponent,
            FileAsStringReaderComponent,
            CommandExecutorComponent,
            StringToFileWriterComponent,
            TcpPortReserverComponent,
        ]:
            TokioParallelRuntimeComponents,
    }
}
