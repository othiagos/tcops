use std::fmt;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum SolverErrorKind {
    Solver,
    Parser,
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

impl From<grb::Error> for SolverError {
    fn from(error: grb::Error) -> Self {
        SolverError::new(
            SolverErrorKind::Solver,
            &format!("Gurobi error: {}", error),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_error_kind_display() {
        assert_eq!(SolverErrorKind::Solver.to_string(), "Solver");
        assert_eq!(SolverErrorKind::Parser.to_string(), "Parser");
        assert_eq!(SolverErrorKind::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_solver_error_new_and_display() {
        let err = SolverError::new(SolverErrorKind::Parser, "Failed to parse file");
        assert_eq!(err.kind, SolverErrorKind::Parser);
        assert_eq!(err.message, "Failed to parse file");
        assert_eq!(err.to_string(), "Failed to parse file");
    }

    #[test]
    fn test_solver_error_from_gurobi_error() {
        let grb_err = grb::Error::ModelObjectRemoved;
        let err = SolverError::from(grb_err);
        assert_eq!(err.kind, SolverErrorKind::Solver);
        assert!(err.message.contains("Gurobi error:"));
    }
}

