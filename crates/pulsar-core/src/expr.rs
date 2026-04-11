/// A node in the Pulsar expression tree.
///
/// This is the primary AST type produced by the parser and consumed by the
/// evaluator. Variants are kept flat (no separate statement type) because
/// Pulsar treats everything as an expression that yields a [`Value`].
///
/// [`Value`]: crate::Value
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // --- Literals ---

    /// Integer literal, e.g. `42`
    Int(i64),

    /// Floating-point literal, e.g. `3.14`
    Float(f64),

    /// Boolean literal: `true` or `false`
    Bool(bool),

    /// String literal, e.g. `"hello"`
    Str(String),

    /// The unit value `()`
    Unit,

    // --- Identifiers ---

    /// Variable reference, e.g. `x`
    Var(String),

    // --- Arithmetic ---

    /// Binary arithmetic or comparison operation
    BinOp {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    /// Unary operation, e.g. `-x` or `!flag`
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expr>,
    },

    // --- Binding ---

    /// `let <name> = <value>` — introduces a variable binding.
    ///
    /// Pulsar treats `let` as an expression that evaluates to `Unit` and
    /// installs the binding in the current session scope.
    Let {
        name: String,
        value: Box<Expr>,
    },

    // --- Control flow ---

    /// `if <cond> { <then> } else { <else_> }`
    ///
    /// Both branches must produce compatible types. The `else` branch is
    /// mandatory so that the expression always has a well-defined value.
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },

    /// A sequence of expressions; evaluates to the last value.
    Block(Vec<Expr>),
}

/// Binary operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Rem,

    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Logical
    And,
    Or,
}

/// Unary operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    /// Arithmetic negation: `-x`
    Neg,
    /// Logical negation: `!x`
    Not,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_let_binding() {
        let expr = Expr::Let {
            name: "x".into(),
            value: Box::new(Expr::Int(42)),
        };
        assert!(matches!(expr, Expr::Let { .. }));
    }

    #[test]
    fn construct_binop() {
        let expr = Expr::BinOp {
            op: BinOp::Add,
            lhs: Box::new(Expr::Int(1)),
            rhs: Box::new(Expr::Int(2)),
        };
        assert!(matches!(expr, Expr::BinOp { op: BinOp::Add, .. }));
    }
}
