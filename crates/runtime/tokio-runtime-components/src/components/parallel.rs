#[cgp::re_export_imports]
mod preset {
    use hermes_prelude::*;
    use hermes_runtime_components::impls::ExecCommandWithNoEnv;
    use hermes_runtime_components::traits::{
        ChildProcessStarterComponent, ChildProcessTypeComponent, ChildProcessWaiterComponent,
        CommandExecutorComponent, CommandWithEnvsExecutorComponent, ConcurrentTaskRunnerComponent,
        DirCreatorComponent, FileAsStringReaderComponent, FileCopierComponent,
        FilePathTypeComponent, RandomGeneratorComponent, SleeperComponent,
        StringToFileWriterComponent, TaskSpawnerComponent, TcpPortReserverComponent, TimeComponent,
    };

    use crate::impls::{
        ProvideStdPathType, ProvideStdTime, ProvideTokioChildProcessType,
        RunParallelTasksWithTokio, StartTokioChildProcess, ThreadRandomGenerator, TokioCopyFile,
        TokioCreateDir, TokioExecCommand, TokioReadFileAsString, TokioReserveTcpPort, TokioSleep,
        TokioSpawnTask, TokioWriteStringToFile, WaitChildProcess,
    };

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
