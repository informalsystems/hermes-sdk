use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::sleep::SleeperComponent;
use ibc_test_components::runtime::traits::child_process::ChildProcessStarterComponent;
use ibc_test_components::runtime::traits::exec_command::CommandExecutorComponent;
use ibc_test_components::runtime::traits::read_file::FileAsStringReaderComponent;
use ibc_test_components::runtime::traits::reserve_port::TcpPortReserverComponent;
use ibc_test_components::runtime::traits::types::child_process::ChildProcessTypeComponent;
use ibc_test_components::runtime::traits::types::file_path::FilePathTypeComponent;
use ibc_test_components::runtime::traits::write_file::StringToFileWriterComponent;
use tokio_runtime_components::components::runtime::TokioRuntimeComponents;

use crate::types::runtime::TokioRuntimeContext;

pub struct RelayerRuntimeComponents;

impl HasComponents for TokioRuntimeContext {
    type Components = RelayerRuntimeComponents;
}

delegate_components!(
    RelayerRuntimeComponents;
    [
        SleeperComponent,
        FilePathTypeComponent,
        ChildProcessTypeComponent,
        ChildProcessStarterComponent,
        FileAsStringReaderComponent,
        CommandExecutorComponent,
        StringToFileWriterComponent,
        TcpPortReserverComponent,
    ]:
        TokioRuntimeComponents,
);
