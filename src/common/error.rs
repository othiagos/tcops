use std::fmt;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum SolverErrorKind {
    Solver,
    Unknown,
}

impl fmt::Display for SolverErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(dead_code)]
pub struct SolverError {
    pub kind: SolverErrorKind,
    pub message: String,
}

impl SolverError {
    pub fn new(kind: SolverErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
