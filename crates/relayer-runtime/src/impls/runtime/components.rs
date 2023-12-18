use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::sleep::SleeperComponent;
use ibc_test_components::runtime::traits::child_process::ChildProcessStarterComponent;
use ibc_test_components::runtime::traits::exec_command::CommandExecutorComponent;
use ibc_test_components::runtime::traits::read_file::FileAsStringReaderComponent;
use ibc_test_components::runtime::traits::reserve_port::TcpPortReserverComponent;
use ibc_test_components::runtime::traits::types::child_process::ChildProcessTypeComponent;
use ibc_test_components::runtime::traits::types::file_path::FilePathTypeComponent;
use ibc_test_components::runtime::traits::write_file::StringToFileWriterComponent;
use tokio_runtime_components::impls::child_process::StartTokioChildProcess;
use tokio_runtime_components::impls::exec_command::TokioExecCommand;
use tokio_runtime_components::impls::read_file::TokioReadFileAsString;
use tokio_runtime_components::impls::reserve_port::TokioReserveTcpPort;
use tokio_runtime_components::impls::sleep::TokioSleep;
use tokio_runtime_components::impls::types::child_process::ProvideTokioChildProcessType;
use tokio_runtime_components::impls::types::file_path::ProvideStdPathType;
use tokio_runtime_components::impls::write_file::TokioWriteStringToFile;

use crate::types::runtime::TokioRuntimeContext;

pub struct TokioRuntimeComponents;

impl HasComponents for TokioRuntimeContext {
    type Components = TokioRuntimeComponents;
}

delegate_components!(
    TokioRuntimeComponents;
    FilePathTypeComponent: ProvideStdPathType,
    ChildProcessTypeComponent: ProvideTokioChildProcessType,
    ChildProcessStarterComponent: StartTokioChildProcess,
    FileAsStringReaderComponent: TokioReadFileAsString,
    CommandExecutorComponent: TokioExecCommand,
    StringToFileWriterComponent: TokioWriteStringToFile,
    TcpPortReserverComponent: TokioReserveTcpPort,
    SleeperComponent: TokioSleep,
);
