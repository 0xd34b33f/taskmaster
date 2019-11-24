use crate::config_reader::{read_config, Task};
use futures::Future;
use log::{debug, error, info, trace, warn};
use nix::sys::stat::{umask, Mode};
use std::borrow::Borrow;
use std::io;
use std::path::PathBuf;
use std::process::{ExitStatus, Stdio};
use tokio::prelude::*;
use tokio::timer;
use tokio_process::{Child, Command, StatusAsync};

struct TaskProps {
    task_info: Task,
    prototype: Command,
    running_copies: Option<Box<dyn Future<Output = Vec<StatusAsync>>>>,
}

//Vec<Box<dyn Future<Output=StatusAsync>>>
fn create_process_prototype(task: &Task) -> Command {
    let cmd: Vec<&str> = task.program_path.split_whitespace().collect();
    let mut program_prototype = Command::new(cmd[0]);
    program_prototype
        .args(&cmd[1..])
        .envs(task.clone().env)
        .stdout(output_builder(&task.stdout, &task.program_name))
        .stderr(output_builder(&task.stderr, &task.program_name))
        .current_dir(&task.woking_dir);

    info!("Created prototype for {}", task.program_name);
    program_prototype
}

async fn spawn_process(props: &mut TaskProps) -> Vec<StatusAsync> {
    let mut spawn_results = vec![];
    while props.task_info.numprocs != 0 {
        match props.prototype.status() {
            Ok(a) => {
                spawn_results.push(a);
            }
            Err(e) => {
                error!("Error spawning  {} : {}", &props.task_info.program_name, e);
            }
        };
        props.task_info.numprocs -= 1;
    }
    spawn_results
}

fn watch_tasks(tasks: Vec<TaskProps>) {}

pub fn mange_tasks(config_path: PathBuf) {
    let mut tasks_props = Vec::<TaskProps>::new();
    let config = read_config(&config_path);
    for task in &config {
        tasks_props.push(TaskProps {
            task_info: task.clone(),
            prototype: create_process_prototype(task),
            running_copies: None,
        });
        for mut task in tasks_props {
            let spawned = spawn_process(&mut task);
            task.running_copies = Some(Box::new(spawned));
        }
    }
}

#[cfg(debug_assertions)]
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
                Stdio::piped()
            }
        };
        return stdoutput;
    }
    Stdio::piped()
}

#[cfg(not(debug_assertions))]
//made this to improve debug
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

#[cfg(test)]
pub mod tests {
    use crate::task_mangment::mange_tasks;
    use std::process::Command;

    #[test]
    pub fn spawn_check() {
        let path = std::env::current_exe().unwrap();
        let root_path = path.ancestors().nth(4).unwrap();
        mange_tasks(root_path.join("server/src/config_reader/test_data/test.yaml"));
        let output = Command::new("/bin/ls").arg("-la").output().unwrap();
        let output_path = "/tmp/out";
        let test_read = std::fs::read(output_path).unwrap();
        if std::path::Path::new(&output_path).exists() {
            std::fs::remove_file(&std::path::Path::new(&output_path))
                .expect("Temporary output file is not deleted");
        }
        assert_eq!(output.stdout, test_read);
    }
}
