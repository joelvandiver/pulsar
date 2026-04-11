use std::collections::HashMap;

use crate::{
    error::PulsarError,
    expr::{BinOp, Expr, UnaryOp},
    value::{EvalResult, Value},
};

// ── Session ───────────────────────────────────────────────────────────────────

/// Persistent state across REPL inputs.
///
/// Pass a `Session` to [`eval`] on every input line; it carries variable
/// bindings forward between calls.
#[derive(Debug, Default, Clone)]
pub struct Session {
    /// Variable bindings installed by `let` expressions.
    pub bindings: HashMap<String, Value>,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Evaluate `expr` in the context of `session`, returning an [`EvalResult`].
///
/// A `let` binding installs the value into `session.bindings` and returns
/// [`EvalResult::Bound`]. All other expressions return [`EvalResult::Ok`].
/// Errors are returned as [`EvalResult::Err`] — this function does not panic.
pub fn eval(expr: &Expr, session: &mut Session) -> EvalResult {
    // Handle `let` at the top level so we can return EvalResult::Bound.
    if let Expr::Let { name, value } = expr {
        return match eval_expr(value, session) {
            Ok(v) => {
                session.bindings.insert(name.clone(), v.clone());
                EvalResult::bound(name.clone(), v)
            }
            Err(e) => EvalResult::Err(e),
        };
    }

    match eval_expr(expr, session) {
        Ok(value) => EvalResult::value(value),
        Err(e) => EvalResult::Err(e),
    }
}

// ── Internal evaluator ────────────────────────────────────────────────────────

fn eval_expr(expr: &Expr, session: &mut Session) -> Result<Value, PulsarError> {
    match expr {
        // Literals evaluate to themselves.
        Expr::Int(n)   => Ok(Value::Int(*n)),
        Expr::Float(f) => Ok(Value::Float(*f)),
        Expr::Bool(b)  => Ok(Value::Bool(*b)),
        Expr::Str(s)   => Ok(Value::Str(s.clone())),
        Expr::Unit     => Ok(Value::Unit),

        // Variable lookup.
        Expr::Var(name) => session
            .bindings
            .get(name)
            .cloned()
            .ok_or_else(|| PulsarError::UndefinedVariable { name: name.clone() }),

        // `let` binding — side-effectful; returns Unit.
        Expr::Let { name, value } => {
            let v = eval_expr(value, session)?;
            session.bindings.insert(name.clone(), v.clone());
            // Callers that need the Bound variant go through `eval()`; the
            // internal path just propagates the value as Unit so blocks work.
            Ok(Value::Unit)
        }

        Expr::UnaryOp { op, operand } => {
            let v = eval_expr(operand, session)?;
            eval_unary(op, v)
        }

        Expr::BinOp { op, lhs, rhs } => {
            let l = eval_expr(lhs, session)?;
            let r = eval_expr(rhs, session)?;
            eval_binary(op, l, r)
        }

        Expr::If { cond, then, else_ } => {
            match eval_expr(cond, session)? {
                Value::Bool(true)  => eval_expr(then, session),
                Value::Bool(false) => eval_expr(else_, session),
                other => Err(PulsarError::TypeError {
                    expected: "bool".into(),
                    found: other.type_name().into(),
                }),
            }
        }

        // A block evaluates each expression in order and returns the last.
        // `let` inside a block still mutates the session (intentional).
        Expr::Block(exprs) => {
            let mut last = Value::Unit;
            for e in exprs {
                last = eval_expr(e, session)?;
            }
            Ok(last)
        }
    }
}

// ── Unary operators ───────────────────────────────────────────────────────────

fn eval_unary(op: &UnaryOp, v: Value) -> Result<Value, PulsarError> {
    match op {
        UnaryOp::Neg => match v {
            Value::Int(n) => n
                .checked_neg()
                .map(Value::Int)
                .ok_or(PulsarError::Overflow),
            Value::Float(f) => Ok(Value::Float(-f)),
            other => Err(PulsarError::TypeError {
                expected: "i64 or f64".into(),
                found: other.type_name().into(),
            }),
        },
        UnaryOp::Not => match v {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            other => Err(PulsarError::TypeError {
                expected: "bool".into(),
                found: other.type_name().into(),
            }),
        },
    }
}

// ── Binary operators ──────────────────────────────────────────────────────────

fn eval_binary(op: &BinOp, l: Value, r: Value) -> Result<Value, PulsarError> {
    match op {
        BinOp::Add => numeric_binop(l, r, i64::checked_add, |a, b| a + b),
        BinOp::Sub => numeric_binop(l, r, i64::checked_sub, |a, b| a - b),
        BinOp::Mul => numeric_binop(l, r, i64::checked_mul, |a, b| a * b),
        BinOp::Div => numeric_div(l, r),
        BinOp::Rem => numeric_rem(l, r),

        BinOp::Eq  => Ok(Value::Bool(values_equal(&l, &r)?)),
        BinOp::Ne  => Ok(Value::Bool(!values_equal(&l, &r)?)),
        BinOp::Lt  => numeric_cmp(l, r, |a, b| a < b, |a, b| a < b),
        BinOp::Le  => numeric_cmp(l, r, |a, b| a <= b, |a, b| a <= b),
        BinOp::Gt  => numeric_cmp(l, r, |a, b| a > b, |a, b| a > b),
        BinOp::Ge  => numeric_cmp(l, r, |a, b| a >= b, |a, b| a >= b),

        BinOp::And => match (l, r) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
            (Value::Bool(_), other) | (other, _) => Err(PulsarError::TypeError {
                expected: "bool".into(),
                found: other.type_name().into(),
            }),
        },
        BinOp::Or => match (l, r) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
            (Value::Bool(_), other) | (other, _) => Err(PulsarError::TypeError {
                expected: "bool".into(),
                found: other.type_name().into(),
            }),
        },
    }
}

/// Applies `int_op` for `(Int, Int)` and `float_op` for `(Float, Float)`.
/// Mixed numeric types are a type error.
fn numeric_binop(
    l: Value,
    r: Value,
    int_op: impl Fn(i64, i64) -> Option<i64>,
    float_op: impl Fn(f64, f64) -> f64,
) -> Result<Value, PulsarError> {
    match (l, r) {
        (Value::Int(a), Value::Int(b)) => int_op(a, b)
            .map(Value::Int)
            .ok_or(PulsarError::Overflow),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(float_op(a, b))),
        (l, r) => Err(type_mismatch(&l, &r)),
    }
}

fn numeric_div(l: Value, r: Value) -> Result<Value, PulsarError> {
    match (l, r) {
        (Value::Int(_), Value::Int(0)) => Err(PulsarError::DivisionByZero),
        (Value::Int(a), Value::Int(b)) => a
            .checked_div(b)
            .map(Value::Int)
            .ok_or(PulsarError::Overflow),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
        (l, r) => Err(type_mismatch(&l, &r)),
    }
}

fn numeric_rem(l: Value, r: Value) -> Result<Value, PulsarError> {
    match (l, r) {
        (Value::Int(_), Value::Int(0)) => Err(PulsarError::DivisionByZero),
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
        (l, r) => Err(type_mismatch(&l, &r)),
    }
}

fn numeric_cmp(
    l: Value,
    r: Value,
    int_cmp: impl Fn(i64, i64) -> bool,
    float_cmp: impl Fn(f64, f64) -> bool,
) -> Result<Value, PulsarError> {
    match (l, r) {
        (Value::Int(a), Value::Int(b))     => Ok(Value::Bool(int_cmp(a, b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(float_cmp(a, b))),
        (l, r) => Err(type_mismatch(&l, &r)),
    }
}

/// Equality is defined for matching types only.
fn values_equal(l: &Value, r: &Value) -> Result<bool, PulsarError> {
    match (l, r) {
        (Value::Int(a),   Value::Int(b))   => Ok(a == b),
        (Value::Float(a), Value::Float(b)) => Ok(a == b),
        (Value::Bool(a),  Value::Bool(b))  => Ok(a == b),
        (Value::Str(a),   Value::Str(b))   => Ok(a == b),
        (Value::Unit,     Value::Unit)     => Ok(true),
        _ => Err(type_mismatch(l, r)),
    }
}

fn type_mismatch(l: &Value, r: &Value) -> PulsarError {
    PulsarError::TypeError {
        expected: l.type_name().into(),
        found: r.type_name().into(),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn run(input: &str) -> EvalResult {
        let expr = parse(input).expect("parse failed");
        eval(&expr, &mut Session::new())
    }

    fn run_with(session: &mut Session, input: &str) -> EvalResult {
        let expr = parse(input).expect("parse failed");
        eval(&expr, session)
    }

    fn val(input: &str) -> Value {
        match run(input) {
            EvalResult::Ok { value, .. } => value,
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    // --- Literals ---

    #[test]
    fn eval_int_literal() {
        assert_eq!(val("42"), Value::Int(42));
    }

    #[test]
    fn eval_float_literal() {
        assert_eq!(val("3.14"), Value::Float(3.14));
    }

    #[test]
    fn eval_bool_literals() {
        assert_eq!(val("true"), Value::Bool(true));
        assert_eq!(val("false"), Value::Bool(false));
    }

    #[test]
    fn eval_string_literal() {
        assert_eq!(val(r#""hello""#), Value::Str("hello".into()));
    }

    #[test]
    fn eval_unit() {
        assert_eq!(val("()"), Value::Unit);
    }

    // --- Arithmetic ---

    #[test]
    fn eval_addition() {
        assert_eq!(val("1 + 2"), Value::Int(3));
    }

    #[test]
    fn eval_subtraction() {
        assert_eq!(val("10 - 3"), Value::Int(7));
    }

    #[test]
    fn eval_multiplication() {
        assert_eq!(val("4 * 5"), Value::Int(20));
    }

    #[test]
    fn eval_division() {
        assert_eq!(val("10 / 2"), Value::Int(5));
    }

    #[test]
    fn eval_remainder() {
        assert_eq!(val("7 % 3"), Value::Int(1));
    }

    #[test]
    fn eval_float_arithmetic() {
        assert_eq!(val("1.5 + 2.5"), Value::Float(4.0));
    }

    #[test]
    fn eval_operator_precedence() {
        assert_eq!(val("2 + 3 * 4"), Value::Int(14));
    }

    #[test]
    fn eval_division_by_zero() {
        assert!(matches!(run("1 / 0"), EvalResult::Err(PulsarError::DivisionByZero)));
    }

    #[test]
    fn eval_overflow() {
        let max = i64::MAX.to_string();
        assert!(matches!(run(&format!("{max} + 1")), EvalResult::Err(PulsarError::Overflow)));
    }

    // --- Unary operators ---

    #[test]
    fn eval_unary_neg_int() {
        assert_eq!(val("-5"), Value::Int(-5));
    }

    #[test]
    fn eval_unary_neg_float() {
        assert_eq!(val("-2.5"), Value::Float(-2.5));
    }

    #[test]
    fn eval_unary_not() {
        assert_eq!(val("!true"), Value::Bool(false));
        assert_eq!(val("!false"), Value::Bool(true));
    }

    // --- Comparisons ---

    #[test]
    fn eval_equality() {
        assert_eq!(val("1 == 1"), Value::Bool(true));
        assert_eq!(val("1 == 2"), Value::Bool(false));
    }

    #[test]
    fn eval_inequality() {
        assert_eq!(val("1 != 2"), Value::Bool(true));
    }

    #[test]
    fn eval_less_than() {
        assert_eq!(val("3 < 5"), Value::Bool(true));
        assert_eq!(val("5 < 3"), Value::Bool(false));
    }

    #[test]
    fn eval_greater_than() {
        assert_eq!(val("5 > 3"), Value::Bool(true));
    }

    // --- Logical operators ---

    #[test]
    fn eval_logical_and() {
        assert_eq!(val("true && false"), Value::Bool(false));
        assert_eq!(val("true && true"),  Value::Bool(true));
    }

    #[test]
    fn eval_logical_or() {
        assert_eq!(val("false || true"), Value::Bool(true));
        assert_eq!(val("false || false"), Value::Bool(false));
    }

    // --- Type errors ---

    #[test]
    fn eval_type_error_mixed_numeric() {
        assert!(matches!(run("1 + 1.0"), EvalResult::Err(PulsarError::TypeError { .. })));
    }

    #[test]
    fn eval_type_error_bool_arithmetic() {
        assert!(matches!(run("true + 1"), EvalResult::Err(PulsarError::TypeError { .. })));
    }

    // --- Variable bindings ---

    #[test]
    fn eval_let_returns_bound() {
        let expr = parse("let x = 10").unwrap();
        let result = eval(&expr, &mut Session::new());
        assert!(matches!(result, EvalResult::Bound { ref name, .. } if name == "x"));
    }

    #[test]
    fn eval_let_persists_in_session() {
        let mut session = Session::new();
        run_with(&mut session, "let x = 42");
        assert_eq!(session.bindings.get("x"), Some(&Value::Int(42)));
    }

    #[test]
    fn eval_var_resolves_from_session() {
        let mut session = Session::new();
        run_with(&mut session, "let x = 7");
        assert_eq!(
            run_with(&mut session, "x * 2"),
            EvalResult::value(Value::Int(14))
        );
    }

    #[test]
    fn eval_undefined_variable() {
        assert!(matches!(
            run("z"),
            EvalResult::Err(PulsarError::UndefinedVariable { name }) if name == "z"
        ));
    }

    // --- If / else ---

    #[test]
    fn eval_if_true_branch() {
        assert_eq!(val("if true { 1 } else { 2 }"), Value::Int(1));
    }

    #[test]
    fn eval_if_false_branch() {
        assert_eq!(val("if false { 1 } else { 2 }"), Value::Int(2));
    }

    #[test]
    fn eval_if_non_bool_condition() {
        assert!(matches!(
            run("if 1 { 2 } else { 3 }"),
            EvalResult::Err(PulsarError::TypeError { .. })
        ));
    }

    // --- Blocks ---

    #[test]
    fn eval_block_returns_last() {
        assert_eq!(val("{ 1; 2; 3 }"), Value::Int(3));
    }

    #[test]
    fn eval_block_let_visible_after() {
        let mut session = Session::new();
        run_with(&mut session, "{ let y = 99 }");
        assert_eq!(session.bindings.get("y"), Some(&Value::Int(99)));
    }
}
