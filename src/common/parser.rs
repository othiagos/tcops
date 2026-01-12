use crate::common::instance::{Cluster, HasId, Instance, Node, Point3, Subgroup, Vehicle};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

const SEC_NODES: &str = "NODE_COORD_SECTION";
const SEC_SUBGROUPS: &str = "SUBGROUP_SECTION";
const SEC_CLUSTERS: &str = "CLUSTER_SECTION";
const SEC_VEHICLES: &str = "VEHICLES_SECTION";

pub fn load_instance(path: &Path) -> Result<Instance, Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut instance = Instance::default();
    read_header(&mut instance, &mut reader)?;
    read_sections(&mut instance, &mut reader)?;

    Ok(instance)
}

fn get_split_line_parts(line: &str) -> Vec<&str> {
    line.trim().split(":").collect()
}

fn parse_integer(value: &str) -> Result<usize, Error> {
    value.parse::<usize>().map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Invalid integer: {} '{}'", e, value),
        )
    })
}

fn parse_float(value: &str) -> Result<f64, Error> {
    value.parse::<f64>().map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Invalid float: {} '{}'", e, value),
        )
    })
}

fn parse_header_string(line_buf: &str) -> Result<String, Error> {
    let parts = get_split_line_parts(line_buf);

    Ok(parts
        .get(1)
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid header"))?
        .trim()
        .to_string())
}

fn parse_header_integer(line_buf: &str) -> Result<usize, Error> {
    let parts = get_split_line_parts(line_buf);

    parts
        .get(1)
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid header"))?
        .trim()
        .parse::<usize>()
        .map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Invalid number: {} '{}'", e, line_buf.trim_end()),
            )
        })
}

fn is_empty_or_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.is_empty() || trimmed.starts_with('#')
}

fn read_next_line(reader: &mut BufReader<File>) -> Result<String, Error> {
    loop {
        let mut line = String::new();

        if reader.read_line(&mut line)? == 0 {
            break;
        }

        if !is_empty_or_comment(&line) {
            return Ok(line);
        }
    }

    Ok("".to_owned())
}

fn ignore_line(reader: &mut BufReader<File>, ignore_lines: usize) -> Result<(), Error> {
    for _ in 0..ignore_lines {
        let mut line = String::new();
        reader.read_line(&mut line)?;
    }

    Ok(())
}

fn read_header(instance: &mut Instance, reader: &mut BufReader<File>) -> Result<(), Error> {
    let mut line = read_next_line(reader)?;
    instance.name = parse_header_string(&line)?;

    ignore_line(reader, 2)?;

    line = read_next_line(reader)?;
    instance.nodes.reserve_exact(parse_header_integer(&line)?);

    line = read_next_line(reader)?;
    instance
        .subgroups
        .reserve_exact(parse_header_integer(&line)?);

    line = read_next_line(reader)?;
    instance
        .clusters
        .reserve_exact(parse_header_integer(&line)?);

    line = read_next_line(reader)?;
    instance
        .vehicles
        .reserve_exact(parse_header_integer(&line)?);

    ignore_line(reader, 1)?;
    Ok(())
}

fn read_sections(instance: &mut Instance, reader: &mut BufReader<File>) -> Result<(), Error> {
    let mut line = String::new();
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }

        let parts = get_split_line_parts(&line);
        let section = parts
            .first()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid section"))?
            .trim();

        if is_empty_or_comment(section) {
            continue;
        }

        match section {
            SEC_NODES => process_nodes(reader, &mut instance.nodes)?,
            SEC_SUBGROUPS => process_subgroups(reader, &mut instance.subgroups, &instance.nodes)?,
            SEC_CLUSTERS => process_clusters(reader, &mut instance.clusters, &instance.subgroups)?,
            SEC_VEHICLES => process_vehicles(reader, &mut instance.vehicles, &instance.nodes)?,
            _ => process_default_section(section)?,
        }
    }

    Ok(())
}

fn process_default_section(section: &str) -> Result<(), Error> {
    Err(Error::new(
        ErrorKind::InvalidData,
        format!("Unknown section {}", section),
    ))
}

fn process_nodes(reader: &mut BufReader<File>, nodes: &mut Vec<Node>) -> Result<(), Error> {
    handle_section(reader, nodes, "Node", parse_node)
}

fn process_subgroups(
    reader: &mut BufReader<File>,
    subgroups: &mut Vec<Subgroup>,
    nodes: &[Node],
) -> Result<(), Error> {
    handle_section(reader, subgroups, "Subgroup", |parts| {
        parse_subgroup(parts, nodes)
    })
}

fn process_clusters(
    reader: &mut BufReader<File>,
    clusters: &mut Vec<Cluster>,
    subgroups: &[Subgroup],
) -> Result<(), Error> {
    handle_section(reader, clusters, "Cluster", |parts| {
        parse_cluster(parts, subgroups)
    })
}

fn process_vehicles(
    reader: &mut BufReader<File>,
    vehicles: &mut Vec<Vehicle>,
    nodes: &[Node],
) -> Result<(), Error> {
    handle_section(reader, vehicles, "Vehicle", |parts| {
        parse_vehicle(parts, nodes)
    })
}

fn validade_section_data_id(section: &str, id: usize, last_id: isize) -> Result<(), Error> {
    if id as isize != last_id + 1 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("{} id {} need be sequential", section, id),
        ));
    }

    Ok(())
}

fn validade_item_id<T>(container: &[T], item_id: usize) -> Result<(), Error>
where
    T: HasId,
{
    let item = container.get(item_id).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Integrity error: Item ID {} does not exist.", item_id),
        )
    })?;

    if item.id() != item_id {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Integrity error: Item ID {} does not exist.", item_id),
        ));
    }

    Ok(())
}

fn handle_section<T, F>(
    reader: &mut BufReader<File>,
    container: &mut Vec<T>,
    section_name: &str,
    parser: F,
) -> Result<(), Error>
where
    T: HasId,
    F: Fn(Vec<&str>) -> Result<T, Error>,
{
    while container.len() < container.capacity() {
        let line = read_next_line(reader)?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        let item = parser(parts)?;

        let last_id = match container.last() {
            Some(n) => n.id() as isize,
            None => -1,
        };

        validade_section_data_id(section_name, item.id(), last_id)?;
        container.insert(item.id(), item);
    }

    Ok(())
}

fn parse_node(parts: Vec<&str>) -> Result<Node, Error> {
    if parts.len() < 4 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid node coordinate: {:?}", parts),
        ));
    }

    let id = parse_integer(parts[0])?;
    let profit = parse_float(parts[1])?;
    let x = parse_float(parts[2])?;
    let y = parse_float(parts[3])?;
    let z = parse_float(parts.get(4).unwrap_or(&"0.0"))?;

    Ok(Node {
        id,
        profit,
        point: Point3 { x, y, z },
    })
}

fn parse_subgroup(parts: Vec<&str>, nodes: &[Node]) -> Result<Subgroup, Error> {
    if parts.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid subgroup data: {:?}", parts),
        ));
    }

    let id = parse_integer(parts[0])?;
    let mut node_ids = Vec::new();
    for part in &parts[1..] {
        let node_id = parse_integer(part)?;

        validade_item_id(nodes, node_id)?;
        node_ids.push(node_id);
    }

    let profit = node_ids.iter().map(|&node_id| nodes[node_id].profit).sum();

    Ok(Subgroup {
        id,
        profit,
        node_ids,
    })
}

fn parse_cluster(parts: Vec<&str>, subgroups: &[Subgroup]) -> Result<Cluster, Error> {
    if parts.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid cluster data"));
    }

    let id = parse_integer(parts[0])?;
    let mut subgroup_ids = Vec::new();
    for part in &parts[1..] {
        let subgroup_id = parse_integer(part)?;

        validade_item_id(subgroups, subgroup_id)?;
        subgroup_ids.push(subgroup_id);
    }

    Ok(Cluster { id, subgroup_ids })
}

fn parse_vehicle(parts: Vec<&str>, nodes: &[Node]) -> Result<Vehicle, Error> {
    if parts.len() < 4 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid vehicle data: {:?}", parts),
        ));
    }

    let id = parse_integer(parts[0])?;
    let tmax = parse_float(parts[1])?;
    let start_node_id = parse_integer(parts[2])?;
    let end_node_id = parse_integer(parts[3])?;

    validade_item_id(nodes, start_node_id)?;
    validade_item_id(nodes, end_node_id)?;

    Ok(Vehicle {
        id,
        tmax,
        start_node_id,
        end_node_id,
    })
}
