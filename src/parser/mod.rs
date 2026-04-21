mod header;
mod linker;
mod sections;
mod utils;
mod validator;

use crate::common::instance::Instance;
use std::fs::File;
use std::io::{BufReader, Error};
use std::path::Path;

pub fn load_instance(path: &Path) -> Result<(Instance, String), Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut instance = Instance::default();
    
    header::read_header(&mut instance, &mut reader)?;
    sections::read_sections(&mut instance, &mut reader)?;
    linker::link_parent_references(&mut instance);

    let input_folder_path = Path::new(&path);
    let input_folder_path = input_folder_path.parent().unwrap_or(Path::new("./"));

    let folder_path = match input_folder_path.to_str() {
        Some(path) => path,
        None => {
            eprintln!("Failed to convert input folder path to string");
            std::process::exit(1);
        }
    };

    Ok((instance, folder_path.to_string()))
}