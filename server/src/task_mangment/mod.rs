use std::path::PathBuf;
use crate::config_reader::{read_config, Task};


struct ProcessInfo {
	pid: u32,
}

struct TaskState {
	task_info: Task,
	procceses: Vec<ProcessInfo>,
}



pub fn mange_tasks(config_path: PathBuf) {
	let tasks = read_config(config_path.as_path());
	
}