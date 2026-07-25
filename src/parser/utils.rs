use std::io::{BufRead, Error, ErrorKind};

pub fn get_split_line_parts(line: &str) -> Vec<&str> {
    line.trim().splitn(2, ':').collect()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_get_split_line_parts() {
        assert_eq!(get_split_line_parts("NAME: burma14"), vec!["NAME", " burma14"]);
        assert_eq!(get_split_line_parts("  KEY : VAL  "), vec!["KEY ", " VAL"]);
        assert_eq!(get_split_line_parts("NO_COLON"), vec!["NO_COLON"]);
        assert_eq!(get_split_line_parts("A:B:C"), vec!["A", "B:C"]);
    }


    #[test]
    fn test_parse_integer() {
        assert_eq!(parse_integer("123").unwrap(), 123);
        assert_eq!(parse_integer("0").unwrap(), 0);
        assert!(parse_integer("-5").is_err());
        assert!(parse_integer("12.34").is_err());
        assert!(parse_integer("abc").is_err());
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(parse_float("12.34").unwrap(), 12.34);
        assert_eq!(parse_float("-5.67").unwrap(), -5.67);
        assert_eq!(parse_float("10").unwrap(), 10.0);
        assert!(parse_float("abc").is_err());
    }

    #[test]
    fn test_is_empty_or_comment() {
        assert!(is_empty_or_comment(""));
        assert!(is_empty_or_comment("   \t\n"));
        assert!(is_empty_or_comment("# comment"));
        assert!(is_empty_or_comment("   # comment with leading spaces"));
        assert!(!is_empty_or_comment("NAME: test"));
    }

    #[test]
    fn test_line_tracker() {
        let input = "
# Comment line 1
NAME: test

# Comment line 2
DIMENSION: 10
";
        let cursor = Cursor::new(input);
        let mut tracker = LineTracker::new(cursor);

        assert_eq!(tracker.current_line(), 0);

        let line1 = tracker.read_next_valid_line().unwrap();
        assert_eq!(line1.trim(), "NAME: test");
        assert_eq!(tracker.current_line(), 3);

        let line2 = tracker.read_next_valid_line().unwrap();
        assert_eq!(line2.trim(), "DIMENSION: 10");
        assert_eq!(tracker.current_line(), 6);

        let eof = tracker.read_next_valid_line().unwrap();
        assert!(eof.is_empty());
    }
}

