use crate::config_reader::{read_config, Task};
use log::{debug, error, info, trace, warn};
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
        programm_prototype.stdout(task.stdout.unwrap());
    }
    Command::new(task.program_name);
}

pub fn mange_tasks(config_path: PathBuf) {
    let tasks = read_config(Stdio::);
    for task in tasks {
        spawn_process(task);
    }
}
