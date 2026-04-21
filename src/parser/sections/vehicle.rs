use crate::common::instance::{Node, Vehicle};
use crate::parser::sections::common::handle_section;
use crate::parser::utils::{LineTracker, parse_float, parse_integer};
use crate::parser::validator::validate_item_id;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

const VEHICLE_DATA_MIN_PARTS: usize = 4;

pub fn process(
    tracker: &mut LineTracker<BufReader<File>>,
    vehicles: &mut Vec<Vehicle>,
    nodes: &[Node],
) -> Result<(), Error> {
    handle_section(tracker, vehicles, "Vehicle", |parts| {
        parse(parts, nodes)
    })
}

fn parse(parts: Vec<&str>, nodes: &[Node]) -> Result<Vehicle, Error> {
    if parts.len() < VEHICLE_DATA_MIN_PARTS {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid vehicle data: {:?}", parts),
        ));
    }

    let id = parse_integer(parts[0])?;
    let budget = parse_float(parts[1])?;
    let start_node_id = parse_integer(parts[2])?;
    let end_node_id = parse_integer(parts[3])?;

    validate_item_id("Node", nodes, start_node_id)?;
    validate_item_id("Node", nodes, end_node_id)?;

    Ok(Vehicle {
        id,
        budget,
        start_node_id,
        end_node_id,
    })
}
