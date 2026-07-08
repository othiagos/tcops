use crate::common::instance::{Node, Point3};
use crate::parser::sections::common::handle_section;
use crate::parser::utils::{LineTracker, parse_float, parse_integer};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

const NODE_DATA_MIN_PARTS: usize = 3;

pub fn process(
    tracker: &mut LineTracker<BufReader<File>>,
    nodes: &mut Vec<Node>,
) -> Result<(), Error> {
    handle_section(tracker, nodes, "Node", parse)
}

fn parse(parts: Vec<&str>) -> Result<Node, Error> {
    if parts.len() < NODE_DATA_MIN_PARTS {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid node coordinate: {:?}", parts),
        ));
    }

    let id = parse_integer(parts[0])?;
    let x = parse_float(parts[1])?;
    let y = parse_float(parts[2])?;
    let z = parse_float(parts.get(3).unwrap_or(&"0.0"))?;

    Ok(Node {
        id,
        point: Point3 { x, y, z },
        parent_subgroup_ids: HashSet::new(),
    })
}
