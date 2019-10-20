use std::path::Path;
use signal::Signal;

pub struct Task<'t> {
	program_name: &'t String,
	program_path: &'t Path,
	numprocs: u16,
	woking_dir: &'t Path,
	autostart: bool,
	autorestart: bool,
	exitcodes: &'t Vec<u8>,
	startretries: u16,
	starttime: u16,
	stopsignal: &'t Signal,
}

pub type task_list<'a> = Vec<Task>;

pub fn read_config()
{}