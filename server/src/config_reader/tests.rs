use crate::config_reader::{read_config, Task};
use signal::Signal;
use std::path::PathBuf;
use std::collections::HashMap;

#[test]
pub fn normal_config() {
	let  path = std::env::current_exe().unwrap();
	let root_path = path.ancestors().nth(4).unwrap();
	println!("Root path: {}", root_path.display());
	let res = read_config(&root_path.join("server/src/config_reader/test_data.yaml"));
	let mut map = HashMap::new();
	map.insert(String::from("STARTED_BY"), String::from("taskmaster"));
	map.insert(String::from("ANSWER"), String::from("42"));
	assert_eq!(
		vec![
			Task {
				program_name: String::from("nginx"),
				program_path: String::from("/usr/local/bin/nginx -c /etc/nginx/test.conf"),
				numprocs: 1,
				umask: 22,
				woking_dir: std::path::PathBuf::from("/tmp"),
				autostart: true,
				autorestart: false,
				exitcodes: vec![0, 2, ],
				startretries: 3,
				starttime: 5,
				stoptime: 10,
				stdout: Some(PathBuf::from("/tmp/nginx.stdout")),
				stderr: Some(PathBuf::from("/tmp/nginx.stderr")),
				stopsignal: Signal::SIGTERM,
				env: map,
			},
			Task {
				program_name: String::from("vogsphere"),
				program_path: String::from("/usr/local/bin/vogsphere-worker --no-prefork"),
				numprocs: 8,
				umask: 77,
				woking_dir: PathBuf::from("/tmp"),
				autostart: true,
				autorestart: false,
				exitcodes: vec![0, ],
				startretries: 3,
				starttime: 5,
				stoptime: 10,
				stdout: Some(PathBuf::from("/tmp/vgsworker.stdout")),
				stderr: Some(PathBuf::from("/tmp/vgsworker.stderr")),
				stopsignal: Signal::SIGUSR1,
				env: HashMap::<String, String>::new(),
			},
		],
		res
	)
}

#[test]
pub fn bad_config() {
	let  path = std::env::current_exe().unwrap();
	let root_path = path.ancestors().nth(4).unwrap();
	let res = read_config(&root_path.join("server/src/config_reader/bad_config.yaml"));
	println!("Result: {:#?}", res);
	assert_eq!(Vec::<Task>::new(), res);
}
