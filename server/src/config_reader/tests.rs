use crate::config_reader::{read_config, Task};
use signal::Signal;
use std::path::PathBuf;

#[test]
pub fn task_list_check() {
    println!("{:?}", std::env::current_exe().unwrap());
    let mut path = std::env::current_exe().unwrap();
    let root_path = path.ancestors().nth(4).unwrap();
    println!("Root path: {}", root_path.display());
    let res = read_config(&root_path.join("server/src/config_reader/test_data.yaml"));
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
                exitcodes: vec![0, 2,],
                startretries: 3,
                starttime: 5,
                stoptime: 10,
                stdout: Some(PathBuf::from("/tmp/nginx.stdout")),
                stderr: Some(PathBuf::from("/tmp/nginx.stderr")),
                stopsignal: Signal::SIGTERM,
            },
            Task {
                program_name: String::from("vogsphere"),
                program_path: String::from("/usr/local/bin/vogsphere-worker --no-prefork"),
                numprocs: 8,
                umask: 77,
                woking_dir: PathBuf::from("/tmp"),
                autostart: true,
                autorestart: false,
                exitcodes: vec![0,],
                startretries: 3,
                starttime: 5,
                stoptime: 10,
                stdout: Some(PathBuf::from("/tmp/vgsworker.stdout"),),
                stderr: Some(PathBuf::from("/tmp/vgsworker.stderr"),),
                stopsignal: Signal::SIGUSR1,
            },
        ],
        res
    )
}
