#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_async_runtime_components::task::impls::concurrent::RunConcurrentTasks;
    pub use hermes_runtime_components::traits::fs::copy_file::FileCopierComponent;
    pub use hermes_runtime_components::traits::fs::create_dir::DirCreatorComponent;
    pub use hermes_runtime_components::traits::fs::file_path::FilePathTypeComponent;
    pub use hermes_runtime_components::traits::fs::read_file::FileAsStringReaderComponent;
    pub use hermes_runtime_components::traits::fs::write_file::StringToFileWriterComponent;
    pub use hermes_runtime_components::traits::os::child_process::{
        ChildProcessStarterComponent, ChildProcessTypeComponent, ChildProcessWaiterComponent,
    };
    pub use hermes_runtime_components::traits::os::exec_command::{
        CommandExecutorComponent, CommandWithEnvsExecutorComponent,
    };
    pub use hermes_runtime_components::traits::os::reserve_port::TcpPortReserverComponent;
    pub use hermes_runtime_components::traits::random::RandomGeneratorComponent;
    pub use hermes_runtime_components::traits::sleep::SleeperComponent;
    pub use hermes_runtime_components::traits::spawn::TaskSpawnerComponent;
    pub use hermes_runtime_components::traits::task::ConcurrentTaskRunnerComponent;
    pub use hermes_runtime_components::traits::time::TimeComponent;

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
