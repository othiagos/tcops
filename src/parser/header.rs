use crate::common::instance::{Instance, Metric};
use crate::parser::utils::{get_split_line_parts, ignore_line, read_next_line};
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

pub fn read_header(instance: &mut Instance, reader: &mut BufReader<File>) -> Result<(), Error> {
    let mut line = read_next_line(reader)?;
    instance.name = parse_header_string(&line)?;

    ignore_line(reader, 2)?;

    line = read_next_line(reader)?;
    instance.nodes.reserve_exact(parse_header_integer(&line)?);

    line = read_next_line(reader)?;
    instance.subgroups.reserve_exact(parse_header_integer(&line)?);

    line = read_next_line(reader)?;
    instance.clusters.reserve_exact(parse_header_integer(&line)?);

    line = read_next_line(reader)?;
    instance.vehicles.reserve_exact(parse_header_integer(&line)?);

    line = read_next_line(reader)?;
    instance.metric = parser_metric(parse_header_string(&line)?)?;

    Ok(())
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

fn parser_metric(metric: String) -> Result<Metric, Error> {
    match metric.as_str() {
        "EUC_2D" => Ok(Metric::Euc2d),
        "EUC_3D" => Ok(Metric::Euc3d),
        "MAN_2D" => Ok(Metric::Man2d),
        "MAN_3D" => Ok(Metric::Man3d),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unknown metric: {}", metric),
        )),
    }
}
