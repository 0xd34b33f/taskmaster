use crate::config_reader::{read_config, Task};
use log::{debug, error, info, trace, warn};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::io;


struct TaskProps {
	task_info: Task,
	prototype: Command,
}

fn create_process_prototype(task: Task) -> Command {
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

fn spawn_process(props: &mut TaskProps) -> Vec<io::Result<Child>> {
	let mut spawn_results = vec![];
	while props.task_info.numprocs != 0 {
		spawn_results.push(props.prototype.spawn());
		props.task_info.numprocs -= 1;
	}
	spawn_results
}

pub fn mange_tasks(config_path: PathBuf) {
	let mut tasks_props = Vec::<TaskProps>::new();
	for task in read_config(&config_path) {
		tasks_props.push(TaskProps {
			task_info: task.clone(),
			prototype: create_process_prototype(task),
		});
	}
	let mut task_states = vec![];
	for mut task in tasks_props {
		for proc in spawn_process(&mut task)
			{
				match proc {
					Ok(a) => task_states.push(a),
					Err(e) => {
						error!("Failed starting {} : {}", task.task_info.program_name, e);
					}
				}
			}
	}
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
		let test_read = std::fs::read("/tmp/out").unwrap();
		assert_eq!(output.stdout, test_read);
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
	return Stdio::piped();
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
	return Stdio::null();
}