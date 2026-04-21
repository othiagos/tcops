use crate::common::instance::{HasId, Instance};
use crate::parser::sections::{cluster, node, subgroup, vehicle};
use crate::parser::utils::{LineTracker, get_split_line_parts, is_empty_or_comment};
use crate::parser::validator::validate_section_data_id;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

const SEC_NODES: &str = "NODE_COORD_SECTION";
const SEC_SUBGROUPS: &str = "SUBGROUP_SECTION";
const SEC_CLUSTERS: &str = "CLUSTER_SECTION";
const SEC_VEHICLES: &str = "VEHICLES_SECTION";

pub fn file_read_sections(
    instance: &mut Instance,
    tracker: &mut LineTracker<BufReader<File>>,
) -> Result<(), Error> {
    loop {
        let line = tracker.read_next_valid_line()?;

        if line.is_empty() {
            break;
        }

        let section = get_section_name(&line)?;

        if is_empty_or_comment(section) {
            continue;
        }

        match section {
            SEC_NODES => node::process(tracker, &mut instance.nodes)?,
            SEC_SUBGROUPS => subgroup::process(tracker, &mut instance.subgroups, &instance.nodes)?,
            SEC_CLUSTERS => cluster::process(tracker, &mut instance.clusters, &instance.subgroups)?,
            SEC_VEHICLES => vehicle::process(tracker, &mut instance.vehicles, &instance.nodes)?,
            _ => process_default_section(section)?,
        }
    }

    Ok(())
}

pub fn handle_section<T, F>(
    tracker: &mut LineTracker<BufReader<File>>,
    container: &mut Vec<T>,
    section_name: &str,
    parser: F,
) -> Result<(), Error>
where
    T: HasId,
    F: Fn(Vec<&str>) -> Result<T, Error>,
{
    while container.len() < container.capacity() {
        let line = tracker.read_next_valid_line()?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        let item = parser(parts)?;

        let last_id = match container.last() {
            Some(n) => n.id() as isize,
            None => -1,
        };

        validate_section_data_id(section_name, item.id(), last_id)?;
        container.insert(item.id(), item);
    }

    Ok(())
}

pub fn get_section_name(line: &str) -> Result<&str, Error> {
    let parts = get_split_line_parts(line);

    Ok(parts
        .first()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid section"))?
        .trim())
}

pub fn process_default_section(section: &str) -> Result<(), Error> {
    Err(Error::new(
        ErrorKind::InvalidData,
        format!("Unknown section {}", section),
    ))
}
