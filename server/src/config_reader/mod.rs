use signal::Signal;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate yaml_rust;

use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

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

impl<'i> TaskList<'i> {}

pub fn create_yaml_structs<'i>(k: &Yaml, v: &Yaml) -> Option<Task<'i>> {
    //	println!("KEY: {:#?} VALUE:{:#?}\n###################################################\n", k, v);
    //	let current_task = Task{
    //		program_name =
    //	};
    let prog_name = match k.as_str() {
        Some(a) => a,
        None => {
            eprintln!("Invalid programm name {:#?}", k);
            std::process::exit(1);
        }
    };
    let programm_params = match v.as_hash() {
        Some(a) => a,
        None => {
            eprintln!("Error parsing body of {}", prog_name);
            return None;
        }
    };
    let cmd = match programm_params.get(&Yaml::String(String::from("cmd"))) {
        Some(a) => match a.as_str() {
            Some(b) => b,
            None => {
                eprintln!("Error parsing {} {:?} cmd path", prog_name, a);
                return None;
            }
        },
        None => {
            eprintln!("Error parsing cmd path for {}", prog_name);
            return None;
        }
    };
    let numprocs = match programm_params.get(&Yaml::String(String::from("numprocs"))) {
        Some(a) => match a.as_i64() {
            Some(b) => b as u16,
            None => {
                eprintln!("Error parsing {} {:?}  numprocs.", prog_name, a);
                1u16
            }
        },
        None => {
            eprintln!("No numprocs for {} is given. Using default 1", prog_name);
            1u16
        }
    };
    println!(
        "PROGNAME: {} CMD: {} NUMPROCS: {}",
        prog_name, cmd, numprocs
    );
    None
}

pub fn read_config(config_path: &Path) {
    let mut file = File::open(config_path);
    let mut file = match file {
        Ok(f) => f,
        Err(er) => {
            eprintln!("Error opening {}", config_path.display());
            std::process::exit(1);
        }
    };
    let mut file_data: String = String::new();
    file.read_to_string(&mut file_data).expect("Empty file");
    let d = YamlLoader::load_from_str(&file_data).expect("empty file");
    let document = &d[0].as_hash().expect("Unwrap of YAML failed");
    let root_element = document.get(&Yaml::String(String::from("programs")));
    let mut root_element = match root_element {
        Some(a) => a,
        None => {
            eprintln!("Root element not found.");
            std::process::exit(1);
        }
    };
    let mut root_map = root_element.as_hash().unwrap();
    for (k, v) in root_map {
        create_yaml_structs(k, v);
    }
}
//	println!("{:#?}", root_map);

//#[cfg(test)]
pub fn task_list_check() {
    let path =
        String::from("/Users/qhetting/DEV/rust/taskmaster/server/src/config_reader/test_data.yaml");
    read_config(Path::new(&path));
}
