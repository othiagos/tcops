use crate::common::instance::{Node, Subgroup};
use crate::parser::sections::common::handle_section;
use crate::parser::utils::{LineTracker, parse_integer, parse_float};
use crate::parser::validator::validate_item_id;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

const SUBGROUP_DATA_MIN_PARTS: usize = 2;

pub fn process(
    tracker: &mut LineTracker<BufReader<File>>,
    subgroups: &mut Vec<Subgroup>,
    nodes: &[Node],
) -> Result<(), Error> {
    handle_section(tracker, subgroups, "Subgroup", |parts| {
        parse(parts, nodes)
    })
}

fn parse(parts: Vec<&str>, nodes: &[Node]) -> Result<Subgroup, Error> {
    if parts.len() < SUBGROUP_DATA_MIN_PARTS {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid subgroup data: {:?}", parts),
        ));
    }

    let id = parse_integer(parts[0])?;
    let profit = parse_float(parts[1])?;
    let mut node_ids = Vec::new();
    for part in &parts[2..] {
        let node_id = parse_integer(part)?;

        validate_item_id("Node", nodes, node_id)?;
        node_ids.push(node_id);
    }

    Ok(Subgroup {
        id,
        profit,
        node_ids,
        parent_cluster_id: 0,
    })
}
