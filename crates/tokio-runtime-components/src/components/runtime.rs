use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::sleep::SleeperComponent;
use ibc_test_components::runtime::traits::child_process::ChildProcessStarterComponent;
use ibc_test_components::runtime::traits::exec_command::CommandExecutorComponent;
use ibc_test_components::runtime::traits::read_file::FileAsStringReaderComponent;
use ibc_test_components::runtime::traits::reserve_port::TcpPortReserverComponent;
use ibc_test_components::runtime::traits::types::child_process::ChildProcessTypeComponent;
use ibc_test_components::runtime::traits::types::file_path::FilePathTypeComponent;
use ibc_test_components::runtime::traits::write_file::StringToFileWriterComponent;

use crate::impls::child_process::StartTokioChildProcess;
use crate::impls::exec_command::TokioExecCommand;
use crate::impls::read_file::TokioReadFileAsString;
use crate::impls::reserve_port::TokioReserveTcpPort;
use crate::impls::sleep::TokioSleep;
use crate::impls::types::child_process::ProvideTokioChildProcessType;
use crate::impls::types::file_path::ProvideStdPathType;
use crate::impls::write_file::TokioWriteStringToFile;

pub struct TokioRuntimeComponents;

delegate_components! {
    #[mark_component(IsTokioRuntimeComponent)]
    #[mark_delegate(DelegatesToTokioRuntimeComponents)]
    TokioRuntimeComponents {
        SleeperComponent: TokioSleep,
        FilePathTypeComponent: ProvideStdPathType,
        ChildProcessTypeComponent: ProvideTokioChildProcessType,
        ChildProcessStarterComponent: StartTokioChildProcess,
        FileAsStringReaderComponent: TokioReadFileAsString,
        CommandExecutorComponent: TokioExecCommand,
        StringToFileWriterComponent: TokioWriteStringToFile,
        TcpPortReserverComponent: TokioReserveTcpPort,
    }
}
