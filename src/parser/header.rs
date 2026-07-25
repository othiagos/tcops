use crate::common::instance::{Instance, Metric};
use crate::parser::utils::{LineTracker, get_split_line_parts};
use std::io::{BufRead, Error, ErrorKind};

pub fn read_header<R: BufRead>(
    instance: &mut Instance,
    tracker: &mut LineTracker<R>,
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

fn read_string<R: BufRead>(
    tracker: &mut LineTracker<R>,
    expected_key: &str,
) -> Result<String, Error> {
    let line = tracker.read_next_valid_line()?;
    let parts = get_split_line_parts(&line);

    if parts.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Expected header '{}: value', but found '{}'", expected_key, line.trim()),
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

fn read_usize<R: BufRead>(
    tracker: &mut LineTracker<R>,
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

fn read_metric<R: BufRead>(tracker: &mut LineTracker<R>, expected_key: &str) -> Result<Metric, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_header_success() {
        let input = "
NAME: test_instance
TYPE: TCOPS
COMMENT: test comment
DIMENSION: 10
SUBGROUPS: 3
CLUSTERS: 2
VEHICLES: 1
EDGE_WEIGHT_TYPE: EUC_2D
";
        let mut instance = Instance::default();
        let mut tracker = LineTracker::new(Cursor::new(input));

        assert!(read_header(&mut instance, &mut tracker).is_ok());
        assert_eq!(instance.name, "test_instance");
        assert_eq!(instance.nodes.capacity(), 10);
        assert_eq!(instance.subgroups.capacity(), 3);
        assert_eq!(instance.clusters.capacity(), 2);
        assert_eq!(instance.vehicles.capacity(), 1);
        assert!(matches!(instance.metric, Metric::Euc2d));
    }

    #[test]
    fn test_read_header_metrics() {
        let metrics = ["EUC_2D", "EUC_3D", "MAN_2D", "MAN_3D"];

        for metric_str in metrics {
            let input = format!(
                "NAME: inst\nTYPE: T\nCOMMENT: C\nDIMENSION: 1\nSUBGROUPS: 1\nCLUSTERS: 1\nVEHICLES: 1\nEDGE_WEIGHT_TYPE: {}\n",
                metric_str
            );
            let mut instance = Instance::default();
            let mut tracker = LineTracker::new(Cursor::new(input));
            assert!(read_header(&mut instance, &mut tracker).is_ok());
        }
    }

    #[test]
    fn test_read_header_invalid_key() {
        let input = "WRONG_KEY: test\n";
        let mut instance = Instance::default();
        let mut tracker = LineTracker::new(Cursor::new(input));
        let err = read_header(&mut instance, &mut tracker);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Expected header 'NAME', but found 'WRONG_KEY'"));
    }

    #[test]
    fn test_read_header_missing_colon() {
        let input = "NAME_NO_COLON\n";
        let mut instance = Instance::default();
        let mut tracker = LineTracker::new(Cursor::new(input));
        let err = read_header(&mut instance, &mut tracker);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Expected header 'NAME: value', but found 'NAME_NO_COLON'"));
    }

    #[test]
    fn test_read_header_invalid_usize() {
        let input = "NAME: inst\nTYPE: T\nCOMMENT: C\nDIMENSION: abc\n";
        let mut instance = Instance::default();
        let mut tracker = LineTracker::new(Cursor::new(input));
        let err = read_header(&mut instance, &mut tracker);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Invalid integer value 'abc'"));
    }

    #[test]
    fn test_read_header_unknown_metric() {
        let input = "NAME: inst\nTYPE: T\nCOMMENT: C\nDIMENSION: 1\nSUBGROUPS: 1\nCLUSTERS: 1\nVEHICLES: 1\nEDGE_WEIGHT_TYPE: UNKNOWN\n";
        let mut instance = Instance::default();
        let mut tracker = LineTracker::new(Cursor::new(input));
        let err = read_header(&mut instance, &mut tracker);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Unknown metric: UNKNOWN"));
    }
}


