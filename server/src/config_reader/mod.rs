use signal::Signal;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate yaml_rust;

use yaml_rust::{YamlEmitter, YamlLoader, Yaml};

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
    let document = &d[0];
    print!("{:#?}", document);
    for &program in document
        {
            
        }
    
    

//    println!("Yaml content:\n{:?}\n", d);
}

//#[cfg(test)]
pub fn task_list_check() {
    let path = String::from("/home/odm3n/dev/taskmaster/server/src/config_reader/test_data.yaml");
    read_config(Path::new(&path));
}
