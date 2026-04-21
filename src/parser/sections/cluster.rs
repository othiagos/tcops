use crate::common::instance::{Cluster, Subgroup};
use crate::parser::sections::common::handle_section;
use crate::parser::utils::{LineTracker, parse_integer};
use crate::parser::validator::validate_item_id;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

const CLUSTER_DATA_MIN_PARTS: usize = 2;

pub fn process(
    tracker: &mut LineTracker<BufReader<File>>,
    clusters: &mut Vec<Cluster>,
    subgroups: &[Subgroup],
) -> Result<(), Error> {
    handle_section(tracker, clusters, "Cluster", |parts| {
        parse(parts, subgroups)
    })
}

fn parse(parts: Vec<&str>, subgroups: &[Subgroup]) -> Result<Cluster, Error> {
    if parts.len() < CLUSTER_DATA_MIN_PARTS {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid cluster data"));
    }

    let id = parse_integer(parts[0])?;
    let mut subgroup_ids = Vec::new();
    for part in &parts[1..] {
        let subgroup_id = parse_integer(part)?;

        validate_item_id("Subgroup", subgroups, subgroup_id)?;
        subgroup_ids.push(subgroup_id);
    }

    Ok(Cluster { id, subgroup_ids })
}
