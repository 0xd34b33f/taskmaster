use crate::config_reader::{read_config, Task};
use log::{debug, error, info, trace, warn};
use std::borrow::Borrow;
use std::io::stdout;
use std::path::PathBuf;
use std::process::{Command, Stdio};

struct ProcessInfo {
    pid: u32,
}

struct TaskState {
    task_info: Task,
    procceses: Vec<ProcessInfo>,
}

fn spawn_process(task: Task) {
    debug!("Started spawning {}", task.program_name);
    let cmd = task.program_path.split_whitespace();
    let mut programm_prototype = Command::new(task.program_name).args(cmd).envs(task.env);
    if task.stdout.is_some() {
        let stdoutput = match std::fs::File::open(&task.stdout.unwrap()) {
            Ok(f) => Stdio::from(f),
            Err(e) => {
                error!(
                    "Error opening {} as stdout for {} : {}",
                    task.stdout.unwrap().display(),
                    task.program_name,
                    e
                );
                warn!("Setting default  value for stdout.");
                std::process::Stdio::piped()
            }
        };
        programm_prototype.stdout(stdoutput);
    }
}

pub fn mange_tasks(config_path: PathBuf) {
    let tasks = read_config(&config_path);
    for task in tasks {
        spawn_process(task);
    }
}
