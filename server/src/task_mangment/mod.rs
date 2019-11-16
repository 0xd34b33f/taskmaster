use crate::config_reader::{read_config, Task};
use log::{debug, error, info, trace, warn};
use std::io;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

struct ProcessInfo {
    pid: u32,
}

struct TaskState {
    task_info: Task,
    procceses: Vec<ProcessInfo>,
}

fn spawn_process(task: Task) {
    let cmd: Vec<&str> = task.program_path.split_whitespace().collect();
    let mut programm_prototype = Command::new(cmd[0]);
    programm_prototype
        .args(&cmd[1..])
        .envs(task.clone().env)
        .stdout(output_builder(&task.stdout, &task.program_name))
        .stderr(output_builder(&task.stderr, &task.program_name));
    debug!("Started spawning {}", task.program_name);
    println!("{:#?}", programm_prototype);
    let handle = programm_prototype.spawn();
    match handle {
        Ok(h) => info!("Successfully spawned {:#?}", h),
        Err(e) => {
            error!("Error spawning {} : {}.", &task.program_name, e);
            info!("Task properties: {:#?}", task.clone());
        }
    };
}

fn output_builder(out: &Option<PathBuf>, name: &str) -> Stdio {
    if let Some(stdout) = out {
        let stdoutput = match std::fs::File::create(&stdout) {
            Ok(f) => Stdio::from(f),
            Err(e) => {
                error!(
                    "Error opening {} as stdout/stderr for {} : {}",
                    stdout.display(),
                    name,
                    e
                );
              return Stdio::null();
            }
        };
        return stdoutput;
    }
    Stdio::null()
}

pub fn mange_tasks(config_path: PathBuf) {
    let tasks = read_config(&config_path);
    for task in tasks {
        spawn_process(task);
    }
}
