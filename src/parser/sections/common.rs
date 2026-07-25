use crate::common::instance::{HasId, Instance};
use crate::parser::sections::{cluster, node, subgroup, vehicle};
use crate::parser::utils::{LineTracker, get_split_line_parts, is_empty_or_comment};
use crate::parser::validator::validate_section_data_id;
use std::io::{BufRead, Error, ErrorKind};

const SEC_NODES: &str = "NODE_COORD_SECTION";
const SEC_SUBGROUPS: &str = "SUBGROUP_SECTION";
const SEC_CLUSTERS: &str = "CLUSTER_SECTION";
const SEC_VEHICLES: &str = "VEHICLES_SECTION";

pub fn file_read_sections<R: BufRead>(
    instance: &mut Instance,
    tracker: &mut LineTracker<R>,
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

pub fn handle_section<T, F, R>(
    tracker: &mut LineTracker<R>,
    container: &mut Vec<T>,
    section_name: &str,
    parser: F,
) -> Result<(), Error>
where
    T: HasId,
    F: Fn(Vec<&str>) -> Result<T, Error>,
    R: BufRead,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_get_section_name() {
        assert_eq!(get_section_name("NODE_COORD_SECTION: id profit x y").unwrap(), "NODE_COORD_SECTION");
        assert_eq!(get_section_name("  CLUSTER_SECTION  ").unwrap(), "CLUSTER_SECTION");
    }

    #[test]
    fn test_process_default_section() {
        let err = process_default_section("UNKNOWN");
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Unknown section UNKNOWN"));
    }

    #[test]
    fn test_file_read_sections_unknown_section() {
        let input = "UNKNOWN_SECTION: header\n";
        let mut instance = Instance::default();
        let mut tracker = LineTracker::new(Cursor::new(input));

        let err = file_read_sections(&mut instance, &mut tracker);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Unknown section UNKNOWN_SECTION"));
    }

    #[test]
    fn test_file_read_sections_full_sequence() {
        let input = "
NODE_COORD_SECTION: id profit x y
0 0.0 1.0 2.0
1 10.0 3.0 4.0

SUBGROUP_SECTION: subgroup_id id-vertex-list
0 5.0 0 1

CLUSTER_SECTION: cluster_id id-subgroup-list
0 0

VEHICLES_SECTION: id tmax start end
0 100.0 0 1
";
        let mut instance = Instance::default();
        instance.nodes.reserve_exact(2);
        instance.subgroups.reserve_exact(1);
        instance.clusters.reserve_exact(1);
        instance.vehicles.reserve_exact(1);

        let mut tracker = LineTracker::new(Cursor::new(input));

        assert!(file_read_sections(&mut instance, &mut tracker).is_ok());
        assert_eq!(instance.nodes.len(), 2);
        assert_eq!(instance.subgroups.len(), 1);
        assert_eq!(instance.clusters.len(), 1);
        assert_eq!(instance.vehicles.len(), 1);
    }
}

