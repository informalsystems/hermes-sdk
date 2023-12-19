use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::mutex::MutexComponent;
use ibc_relayer_components::runtime::traits::sleep::SleeperComponent;
use ibc_relayer_components::runtime::traits::stream::{StreamMapperComponent, StreamTypeComponent};
use ibc_relayer_components::runtime::traits::task::ConcurrentTaskRunnerComponent;
use ibc_test_components::runtime::traits::child_process::ChildProcessStarterComponent;
use ibc_test_components::runtime::traits::exec_command::CommandExecutorComponent;
use ibc_test_components::runtime::traits::read_file::FileAsStringReaderComponent;
use ibc_test_components::runtime::traits::reserve_port::TcpPortReserverComponent;
use ibc_test_components::runtime::traits::types::child_process::ChildProcessTypeComponent;
use ibc_test_components::runtime::traits::types::file_path::FilePathTypeComponent;
use ibc_test_components::runtime::traits::write_file::StringToFileWriterComponent;

use crate::components::parallel::TokioParallelRuntimeComponents;
use crate::impls::concurrent_task::RunConcurrentTasks;

pub struct TokioConcurrentRuntimeComponents;

delegate_components! {
    #[mark_component(IsTokioConcurrentRuntimeComponent)]
    #[mark_delegate(DelegatesToTokioConcurrentRuntimeComponents)]
    TokioConcurrentRuntimeComponents {
        ConcurrentTaskRunnerComponent: RunConcurrentTasks,
        [
            SleeperComponent,
            MutexComponent,
            StreamTypeComponent,
            StreamMapperComponent,
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
