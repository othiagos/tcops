use std::io::{BufRead, Error, ErrorKind};

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

pub struct LineTracker<R> {
    reader: R,
    current_line: usize,
}

impl<T: BufRead> LineTracker<T> {
    pub fn new(reader: T) -> Self {
        Self {
            reader,
            current_line: 0,
        }
    }

    pub fn current_line(&self) -> usize {
        self.current_line
    }

    pub fn read_next_valid_line(&mut self) -> Result<String, Error> {
        loop {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line)?;

            if bytes_read == 0 {
                return Ok(String::new());
            }

            self.current_line += 1;

            if !is_empty_or_comment(&line) {
                return Ok(line);
            }
        }
    }
}
