#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_runtime_components::impls::os::exec_command::ExecCommandWithNoEnv;
    use hermes_runtime_components::traits::fs::copy_file::FileCopierComponent;
    use hermes_runtime_components::traits::fs::create_dir::DirCreatorComponent;
    use hermes_runtime_components::traits::fs::file_path::FilePathTypeComponent;
    use hermes_runtime_components::traits::fs::read_file::FileAsStringReaderComponent;
    use hermes_runtime_components::traits::fs::write_file::StringToFileWriterComponent;
    use hermes_runtime_components::traits::os::child_process::{
        ChildProcessStarterComponent, ChildProcessTypeComponent, ChildProcessWaiterComponent,
    };
    use hermes_runtime_components::traits::os::exec_command::{
        CommandExecutorComponent, CommandWithEnvsExecutorComponent,
    };
    use hermes_runtime_components::traits::os::reserve_port::TcpPortReserverComponent;
    use hermes_runtime_components::traits::random::RandomGeneratorComponent;
    use hermes_runtime_components::traits::sleep::SleeperComponent;
    use hermes_runtime_components::traits::spawn::TaskSpawnerComponent;
    use hermes_runtime_components::traits::task::ConcurrentTaskRunnerComponent;
    use hermes_runtime_components::traits::time::TimeComponent;

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
    use crate::impls::parallel_task::RunParallelTasksWithTokio;
    use crate::impls::random::ThreadRandomGenerator;
    use crate::impls::sleep::TokioSleep;
    use crate::impls::spawn::TokioSpawnTask;
    use crate::impls::time::ProvideStdTime;

    cgp_preset! {
        TokioParallelRuntimeComponents {
            SleeperComponent: TokioSleep,
            TimeComponent: ProvideStdTime,
            ConcurrentTaskRunnerComponent: RunParallelTasksWithTokio,
            TaskSpawnerComponent: TokioSpawnTask,
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
}
