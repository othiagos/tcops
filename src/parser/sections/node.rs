use crate::common::instance::{Node, Point3};
use crate::parser::sections::common::handle_section;
use crate::parser::utils::{LineTracker, parse_float, parse_integer};
use std::collections::HashSet;
use std::io::{BufRead, Error, ErrorKind};

const NODE_DATA_MIN_PARTS: usize = 3;

pub fn process<R: BufRead>(
    tracker: &mut LineTracker<R>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_node_2d() {
        let node = parse(vec!["0", "10.5", "20.5"]).unwrap();
        assert_eq!(node.id, 0);
        assert_eq!(node.point.x, 10.5);
        assert_eq!(node.point.y, 20.5);
        assert_eq!(node.point.z, 0.0);
    }

    #[test]
    fn test_parse_node_3d() {
        let node = parse(vec!["1", "10.5", "20.5", "30.5"]).unwrap();
        assert_eq!(node.id, 1);
        assert_eq!(node.point.x, 10.5);
        assert_eq!(node.point.y, 20.5);
        assert_eq!(node.point.z, 30.5);
    }
    
    #[test]
    fn test_parse_node_5d() {
        let node = parse(vec!["1", "10.5", "20.5", "30.5",  "10.5", "20.5"]).unwrap();
        assert_eq!(node.id, 1);
        assert_eq!(node.point.x, 10.5);
        assert_eq!(node.point.y, 20.5);
        assert_eq!(node.point.z, 30.5);
    }

    #[test]
    fn test_parse_node_insufficient_parts() {
        let err = parse(vec!["0", "10.5"]);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Invalid node coordinate"));
    }

    #[test]
    fn test_process_nodes() {
        let input = "0 1.0 2.0\n1 3.0 4.0 5.0\n";
        let mut tracker = LineTracker::new(Cursor::new(input));
        let mut nodes = Vec::with_capacity(2);

        assert!(process(&mut tracker, &mut nodes).is_ok());
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].id, 0);
        assert_eq!(nodes[1].id, 1);
    }
}

