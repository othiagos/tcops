use crate::common::instance::{Instance, Metric};
use crate::parser::utils::{LineTracker, get_split_line_parts};
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

pub fn read_header(
    instance: &mut Instance,
    tracker: &mut LineTracker<BufReader<File>>,
) -> Result<(), Error> {

    instance.name = read_string(tracker, "NAME")?;
    read_string(tracker, "TYPE")?;
    read_string(tracker, "COMMENT")?;

    instance.nodes.reserve_exact(read_usize(tracker, "DIMENSION")?);
    instance.subgroups.reserve_exact(read_usize(tracker, "SUBGROUPS")?);
    instance.clusters.reserve_exact(read_usize(tracker, "CLUSTERS")?);
    instance.vehicles.reserve_exact(read_usize(tracker, "VEHICLES")?);
    instance.metric = read_metric(tracker, "EDGE_WEIGHT_TYPE")?;

    Ok(())
}

fn read_string(
    tracker: &mut LineTracker<BufReader<File>>,
    expected_key: &str,
) -> Result<String, Error> {
    let line = tracker.read_next_valid_line()?;
    let parts = get_split_line_parts(&line);

    if parts.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Expected header '{}: value', but found '{}'", expected_key, line),
        ));
    }

    let key = parts.first().unwrap_or(&"").trim();
    if key != expected_key {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!( "Expected header '{}', but found '{}'", expected_key, key,
        )));
    }

    Ok(parts.get(1).unwrap_or(&"").trim().to_string())
}

fn read_usize(
    tracker: &mut LineTracker<BufReader<File>>,
    expected_key: &str,
) -> Result<usize, Error> {
    let value = read_string(tracker, expected_key)?;
    
    value.parse::<usize>().map_err(|_| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Invalid integer value '{}'", value),
        )
    })
}

fn read_metric(tracker: &mut LineTracker<BufReader<File>>, expected_key: &str) -> Result<Metric, Error> {
    let metric_str = read_string(tracker, expected_key)?;

    match metric_str.as_str() {
        "EUC_2D" => Ok(Metric::Euc2d),
        "EUC_3D" => Ok(Metric::Euc3d),
        "MAN_2D" => Ok(Metric::Man2d),
        "MAN_3D" => Ok(Metric::Man3d),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unknown metric: {}", metric_str),
        )),
    }
}
