use std::fmt;

/// All errors that can occur within the Pulsar core engine.
#[derive(Debug, Clone, PartialEq)]
pub enum PulsarError {
    /// Input could not be parsed into a valid expression.
    ParseError {
        message: String,
        /// 0-based byte offset in the input where the error occurred, if known.
        offset: Option<usize>,
    },

    /// A well-formed expression could not be evaluated.
    EvalError {
        message: String,
    },

    /// A variable was referenced before it was defined.
    UndefinedVariable {
        name: String,
    },

    /// An operation was attempted on values of incompatible types.
    TypeError {
        expected: String,
        found: String,
    },

    /// Division or remainder by zero.
    DivisionByZero,

    /// Integer or float arithmetic overflowed.
    Overflow,
}

impl fmt::Display for PulsarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PulsarError::ParseError { message, offset: Some(offset) } => {
                write!(f, "parse error at offset {offset}: {message}")
            }
            PulsarError::ParseError { message, offset: None } => {
                write!(f, "parse error: {message}")
            }
            PulsarError::EvalError { message } => {
                write!(f, "eval error: {message}")
            }
            PulsarError::UndefinedVariable { name } => {
                write!(f, "undefined variable `{name}`")
            }
            PulsarError::TypeError { expected, found } => {
                write!(f, "type error: expected {expected}, found {found}")
            }
            PulsarError::DivisionByZero => write!(f, "division by zero"),
            PulsarError::Overflow => write!(f, "arithmetic overflow"),
        }
    }
}

impl std::error::Error for PulsarError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_parse_error_with_offset() {
        let e = PulsarError::ParseError {
            message: "unexpected token".into(),
            offset: Some(5),
        };
        assert_eq!(e.to_string(), "parse error at offset 5: unexpected token");
    }

    #[test]
    fn display_undefined_variable() {
        let e = PulsarError::UndefinedVariable { name: "x".into() };
        assert_eq!(e.to_string(), "undefined variable `x`");
    }

    #[test]
    fn display_type_error() {
        let e = PulsarError::TypeError {
            expected: "Int".into(),
            found: "Bool".into(),
        };
        assert_eq!(e.to_string(), "type error: expected Int, found Bool");
    }
}
