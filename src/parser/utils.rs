use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

pub fn get_split_line_parts(line: &str) -> Vec<&str> {
    line.trim().split(":").collect()
}

pub fn parse_integer(value: &str) -> Result<usize, Error> {
    value.parse::<usize>().map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Invalid integer: {} '{}'", e, value),
        )
    })
}

pub fn parse_float(value: &str) -> Result<f64, Error> {
    value.parse::<f64>().map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Invalid float: {} '{}'", e, value),
        )
    })
}

pub fn is_empty_or_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.is_empty() || trimmed.starts_with('#')
}

pub fn read_next_line(reader: &mut BufReader<File>) -> Result<String, Error> {
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

pub fn ignore_line(reader: &mut BufReader<File>, ignore_lines: usize) -> Result<(), Error> {
    for _ in 0..ignore_lines {
        let mut line = String::new();
        reader.read_line(&mut line)?;
    }

    Ok(())
}
