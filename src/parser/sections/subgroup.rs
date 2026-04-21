use crate::common::instance::{Node, Subgroup};
use crate::parser::sections::common::handle_section;
use crate::parser::utils::parse_integer;
use crate::parser::validator::validate_item_id;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

const SUBGROUP_DATA_MIN_PARTS: usize = 2;

pub fn process_subgroups(
    reader: &mut BufReader<File>,
    subgroups: &mut Vec<Subgroup>,
    nodes: &[Node],
) -> Result<(), Error> {
    handle_section(reader, subgroups, "Subgroup", |parts| {
        parse_subgroup(parts, nodes)
    })
}

fn parse_subgroup(parts: Vec<&str>, nodes: &[Node]) -> Result<Subgroup, Error> {
    if parts.len() < SUBGROUP_DATA_MIN_PARTS {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid subgroup data: {:?}", parts),
        ));
    }

    let id = parse_integer(parts[0])?;
    let mut node_ids = Vec::new();
    for part in &parts[1..] {
        let node_id = parse_integer(part)?;

        validate_item_id(nodes, node_id)?;
        node_ids.push(node_id);
    }

    let profit = node_ids.iter().map(|&node_id| nodes[node_id].profit).sum();

    Ok(Subgroup {
        id,
        profit,
        node_ids,
        parent_cluster_id: 0,
    })
}