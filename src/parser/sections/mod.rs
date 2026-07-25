pub mod cluster;
pub mod common;
pub mod node;
pub mod subgroup;
pub mod vehicle;

use crate::common::instance::Instance;
use crate::parser::sections::common::file_read_sections;
use crate::parser::utils::LineTracker;
use std::io::{BufRead, Error};

pub fn read_sections<R: BufRead>(
    instance: &mut Instance,
    tracker: &mut LineTracker<R>,
) -> Result<(), Error> {
    file_read_sections(instance, tracker)
}

