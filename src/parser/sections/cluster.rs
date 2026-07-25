use crate::common::instance::{Cluster, Subgroup};
use crate::parser::sections::common::handle_section;
use crate::parser::utils::{LineTracker, parse_integer};
use crate::parser::validator::validate_item_id;
use std::io::{BufRead, Error, ErrorKind};

const CLUSTER_DATA_MIN_PARTS: usize = 2;

pub fn process<R: BufRead>(
    tracker: &mut LineTracker<R>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_cluster_success() {
        let subgroups = vec![
            Subgroup { id: 0, ..Default::default() },
            Subgroup { id: 1, ..Default::default() },
        ];

        let cluster = parse(vec!["0", "0", "1"], &subgroups).unwrap();
        assert_eq!(cluster.id, 0);
        assert_eq!(cluster.subgroup_ids, vec![0, 1]);
    }

    #[test]
    fn test_parse_cluster_invalid_subgroup_ref() {
        let subgroups = vec![Subgroup { id: 0, ..Default::default() }];

        let err = parse(vec!["0", "0", "5"], &subgroups);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Integrity error: Subgroup ID 5 does not exist."));
    }

    #[test]
    fn test_parse_cluster_insufficient_parts() {
        let subgroups = vec![];
        let err = parse(vec!["0"], &subgroups);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Invalid cluster data"));
    }

    #[test]
    fn test_process_clusters() {
        let subgroups = vec![
            Subgroup { id: 0, ..Default::default() },
        ];
        let input = "0 0\n";
        let mut tracker = LineTracker::new(Cursor::new(input));
        let mut clusters = Vec::with_capacity(1);

        assert!(process(&mut tracker, &mut clusters, &subgroups).is_ok());
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].id, 0);
        assert_eq!(clusters[0].subgroup_ids, vec![0]);
    }
}

