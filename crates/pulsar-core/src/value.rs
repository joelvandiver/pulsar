use std::fmt;

use crate::PulsarError;

/// A runtime value produced by evaluating an [`Expr`].
///
/// [`Expr`]: crate::Expr
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Unit,
}

impl Value {
    /// The name of this value's type, as displayed to the user.
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "i64",
            Value::Float(_) => "f64",
            Value::Bool(_) => "bool",
            Value::Str(_) => "String",
            Value::Unit => "()",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Str(s) => write!(f, "{s}"),
            Value::Unit => write!(f, "()"),
        }
    }
}

/// The outcome of evaluating a single input in a Pulsar session.
#[derive(Debug, Clone, PartialEq)]
pub enum EvalResult {
    /// Evaluation succeeded, yielding a value and its type name.
    Ok {
        value: Value,
        /// Human-readable type, e.g. `"i64"` or `"bool"`.
        type_name: String,
    },

    /// The expression was a `let` binding; the name was added to scope.
    Bound {
        name: String,
        value: Value,
        type_name: String,
    },

    /// Evaluation failed.
    Err(PulsarError),
}

impl EvalResult {
    /// Convenience constructor for a successful value result.
    pub fn value(v: Value) -> Self {
        let type_name = v.type_name().to_string();
        EvalResult::Ok { value: v, type_name }
    }

    /// Convenience constructor for a successful let-binding result.
    pub fn bound(name: impl Into<String>, v: Value) -> Self {
        let type_name = v.type_name().to_string();
        EvalResult::Bound { name: name.into(), value: v, type_name }
    }

    /// Returns `true` if this result represents an error.
    pub fn is_err(&self) -> bool {
        matches!(self, EvalResult::Err(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_type_names() {
        assert_eq!(Value::Int(0).type_name(), "i64");
        assert_eq!(Value::Float(0.0).type_name(), "f64");
        assert_eq!(Value::Bool(true).type_name(), "bool");
        assert_eq!(Value::Str(String::new()).type_name(), "String");
        assert_eq!(Value::Unit.type_name(), "()");
    }

    #[test]
    fn eval_result_value_constructor() {
        let r = EvalResult::value(Value::Int(42));
        assert!(matches!(r, EvalResult::Ok { type_name, .. } if type_name == "i64"));
    }

    #[test]
    fn eval_result_bound_constructor() {
        let r = EvalResult::bound("x", Value::Bool(true));
        assert!(matches!(r, EvalResult::Bound { name, type_name, .. }
            if name == "x" && type_name == "bool"));
    }

    #[test]
    fn eval_result_is_err() {
        let r = EvalResult::Err(PulsarError::DivisionByZero);
        assert!(r.is_err());

        let r = EvalResult::value(Value::Unit);
        assert!(!r.is_err());
    }

    #[test]
    fn value_display() {
        assert_eq!(Value::Int(7).to_string(), "7");
        assert_eq!(Value::Bool(false).to_string(), "false");
        assert_eq!(Value::Str("hi".into()).to_string(), "hi");
        assert_eq!(Value::Unit.to_string(), "()");
    }
}
