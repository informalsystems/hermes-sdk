#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_async_runtime_components::task::impls::RunConcurrentTasks;
    pub use hermes_runtime_components::traits::{
        ChildProcessStarterComponent, ChildProcessTypeComponent, ChildProcessWaiterComponent,
        CommandExecutorComponent, CommandWithEnvsExecutorComponent, ConcurrentTaskRunnerComponent,
        DirCreatorComponent, FileAsStringReaderComponent, FileCopierComponent,
        FilePathTypeComponent, RandomGeneratorComponent, SleeperComponent,
        StringToFileWriterComponent, TaskSpawnerComponent, TcpPortReserverComponent, TimeComponent,
    };

    use crate::components::parallel::TokioParallelRuntimeComponents;

    cgp_preset! {
        TokioConcurrentRuntimeComponents {
            ConcurrentTaskRunnerComponent: RunConcurrentTasks,
            [
                SleeperComponent,
                TimeComponent,
                TaskSpawnerComponent,
                FilePathTypeComponent,
                ChildProcessTypeComponent,
                ChildProcessStarterComponent,
                ChildProcessWaiterComponent,
                FileAsStringReaderComponent,
                DirCreatorComponent,
                FileCopierComponent,
                CommandExecutorComponent,
                CommandWithEnvsExecutorComponent,
                StringToFileWriterComponent,
                TcpPortReserverComponent,
                RandomGeneratorComponent,
            ]:
                TokioParallelRuntimeComponents::Provider,
        }
    }
}
