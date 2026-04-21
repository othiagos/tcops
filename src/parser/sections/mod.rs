pub mod cluster;
pub mod common;
pub mod node;
pub mod subgroup;
pub mod vehicle;

use crate::common::instance::Instance;
use crate::parser::sections::common::file_read_sections;
use std::fs::File;
use std::io::{BufReader, Error};

pub fn read_sections(instance: &mut Instance, reader: &mut BufReader<File>) -> Result<(), Error> {
    file_read_sections(instance, reader)
}
