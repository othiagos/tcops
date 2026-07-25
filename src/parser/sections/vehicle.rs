use crate::common::instance::{Node, Vehicle};
use crate::parser::sections::common::handle_section;
use crate::parser::utils::{LineTracker, parse_float, parse_integer};
use crate::parser::validator::validate_item_id;
use std::io::{BufRead, Error, ErrorKind};

const VEHICLE_DATA_MIN_PARTS: usize = 4;

pub fn process<R: BufRead>(
    tracker: &mut LineTracker<R>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_vehicle_success() {
        let nodes = vec![
            Node { id: 0, ..Default::default() },
            Node { id: 1, ..Default::default() },
        ];

        let vehicle = parse(vec!["0", "500.0", "0", "1"], &nodes).unwrap();
        assert_eq!(vehicle.id, 0);
        assert_eq!(vehicle.budget, 500.0);
        assert_eq!(vehicle.start_node_id, 0);
        assert_eq!(vehicle.end_node_id, 1);
    }

    #[test]
    fn test_parse_vehicle_invalid_node_ref() {
        let nodes = vec![Node { id: 0, ..Default::default() }];

        let err = parse(vec!["0", "500.0", "0", "2"], &nodes);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Integrity error: Node ID 2 does not exist."));
    }

    #[test]
    fn test_parse_vehicle_insufficient_parts() {
        let nodes = vec![];
        let err = parse(vec!["0", "500.0", "0"], &nodes);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Invalid vehicle data"));
    }

    #[test]
    fn test_process_vehicles() {
        let nodes = vec![
            Node { id: 0, ..Default::default() },
        ];
        let input = "0 800.0 0 0\n";
        let mut tracker = LineTracker::new(Cursor::new(input));
        let mut vehicles = Vec::with_capacity(1);

        assert!(process(&mut tracker, &mut vehicles, &nodes).is_ok());
        assert_eq!(vehicles.len(), 1);
        assert_eq!(vehicles[0].id, 0);
        assert_eq!(vehicles[0].budget, 800.0);
    }
}

