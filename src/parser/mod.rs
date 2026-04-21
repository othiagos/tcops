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
    let reader = BufReader::new(file);

    let mut tracker = utils::LineTracker::new(reader);
    let mut instance = Instance::default();

    header::read_header(&mut instance, &mut tracker)
        .map_err(|e| format_err(tracker.current_line(), e))?;

    sections::read_sections(&mut instance, &mut tracker)
        .map_err(|e| format_err(tracker.current_line(), e))?;

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

fn format_err(line_num: usize, e: Error) -> Error {
    Error::new(e.kind(), format!("Error on line {}: {}", line_num, e))
}