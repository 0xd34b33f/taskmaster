use std::path::Path;
use signal::Signal;
use serde_yaml::from_reader;
use std::fs::File;
use std::io::prelude::*;

pub struct Task<'i> {
	program_name: String,
	program_path: &'i Path,
	numprocs: u16,
	woking_dir: &'i Path,
	autostart: bool,
	autorestart: bool,
	exitcodes: Vec<u8>,
	startretries: u16,
	starttime: u16,
	stopsignal: Signal,
}

pub struct TaskList<'i>(Vec<Task<'i>>);

impl<'i> TaskList<'i> {
	pub fn read_config(&mut self, config_path: &Path)
	{
		let mut file = File::open(config_path);
		let file = match file {
			Ok(f) => f,
			Err(er) => {
				eprintln!("Error opening {}", config_path.display());
				std::process::exit(1);
			}
		};
		let parse_res = from_reader(file);
		let parse_res:String = match parse_res {
			Ok(r) => parse_res,
			Err(e) => {
				eprintln!("Error parsing {}", config_path.display());
				std::process::exit(1);
			}
		};
	}
}

