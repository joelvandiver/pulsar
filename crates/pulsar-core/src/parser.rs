/// Pratt parser — converts a token stream into an [`Expr`] AST.
///
/// Operator precedence (low → high):
///   1. `||`
///   2. `&&`
///   3. `==`  `!=`
///   4. `<`  `<=`  `>`  `>=`
///   5. `+`  `-`
///   6. `*`  `/`  `%`
///   7. Unary `-`  `!`
///   8. Atoms: literals, identifiers, `(expr)`, blocks, `if`, `let`
use crate::{
    error::PulsarError,
    expr::{BinOp, Expr, UnaryOp},
    lexer::{lex, Token},
};

// ── Public entry point ────────────────────────────────────────────────────────

/// Parse a complete input string into an [`Expr`].
///
/// Returns `Err(PulsarError::ParseError { .. })` on failure.
/// If the input is syntactically incomplete (e.g. an unclosed brace), the
/// error message contains the sentinel text `"incomplete input"` so the REPL
/// frontend can detect it and prompt for more input.
pub fn parse(input: &str) -> Result<Expr, PulsarError> {
    let tokens = lex(input).map_err(|offset| PulsarError::ParseError {
        message: format!("unexpected character at offset {offset}"),
        offset: Some(offset),
    })?;

    // Strip leading/trailing newlines — they are only meaningful as statement
    // separators inside blocks.
    let tokens: Vec<_> = tokens
        .into_iter()
        .map(|(t, r)| (t, r))
        .collect();

    let mut p = Parser::new(tokens);

    // A top-level input is either a block body (multiple semi/newline-separated
    // expressions) or a single expression.
    let expr = p.parse_block_body(/*top_level=*/true)?;

    if !p.is_at_end() {
        let offset = p.current_offset();
        return Err(PulsarError::ParseError {
            message: "unexpected token after expression".into(),
            offset: Some(offset),
        });
    }

    Ok(expr)
}

// ── Parser state ──────────────────────────────────────────────────────────────

struct Parser {
    tokens: Vec<(Token, std::ops::Range<usize>)>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<(Token, std::ops::Range<usize>)>) -> Self {
        // Drop bare newlines that appear at the very start/end to keep
        // top-level parsing clean.
        Self { tokens, pos: 0 }
    }

    // ── Cursor helpers ────────────────────────────────────────────────────────

    fn peek(&self) -> Option<&Token> {
        // Skip newline tokens when peeking (they only matter as separators
        // inside block bodies, handled there explicitly).
        let mut i = self.pos;
        while i < self.tokens.len() {
            if self.tokens[i].0 != Token::Newline {
                return Some(&self.tokens[i].0);
            }
            i += 1;
        }
        None
    }

    /// Peek without skipping newlines — used inside block bodies.
    fn peek_raw(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|(t, _)| t)
    }

    fn advance(&mut self) -> Option<&Token> {
        while self.pos < self.tokens.len() {
            let tok = &self.tokens[self.pos].0;
            self.pos += 1;
            if *tok != Token::Newline {
                return Some(&self.tokens[self.pos - 1].0);
            }
        }
        None
    }

    /// Advance one raw token (including newlines).
    fn advance_raw(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
            Some(&self.tokens[self.pos - 1].0)
        } else {
            None
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), PulsarError> {
        match self.peek() {
            Some(t) if t == expected => { self.advance(); Ok(()) }
            Some(_) => Err(self.error(format!("expected {expected:?}"))),
            None => Err(self.incomplete()),
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().is_none()
    }

    fn current_offset(&self) -> usize {
        self.tokens.get(self.pos).map(|(_, r)| r.start).unwrap_or(0)
    }

    fn error(&self, message: impl Into<String>) -> PulsarError {
        PulsarError::ParseError {
            message: message.into(),
            offset: Some(self.current_offset()),
        }
    }

    fn incomplete(&self) -> PulsarError {
        PulsarError::ParseError {
            message: "incomplete input".into(),
            offset: None,
        }
    }

    // ── Grammar ───────────────────────────────────────────────────────────────

    /// Parse a sequence of expressions separated by `;` or newlines.
    /// `top_level = true` means we are not inside `{ }`.
    fn parse_block_body(&mut self, top_level: bool) -> Result<Expr, PulsarError> {
        let mut exprs: Vec<Expr> = Vec::new();

        loop {
            // Skip any leading / consecutive separators.
            self.skip_separators();

            // Check for end of block / end of input.
            match self.peek_raw() {
                None => break,
                Some(Token::RBrace) if !top_level => break,
                _ => {}
            }

            let expr = self.parse_expr()?;
            exprs.push(expr);

            // After an expression we must see a separator, a close-brace, or
            // end-of-input.  A separator means there may be more expressions;
            // anything else terminates the block body.
            match self.peek_raw() {
                None => break,
                Some(Token::RBrace) if !top_level => break,
                Some(Token::Semi) | Some(Token::Newline) => {
                    // Leave the separator in place; skip_separators at the top
                    // of the next iteration will consume it (and any extras).
                }
                _ => break,
            }
        }

        match exprs.len() {
            0 => Ok(Expr::Unit),
            1 => Ok(exprs.remove(0)),
            _ => Ok(Expr::Block(exprs)),
        }
    }

    fn skip_separators(&mut self) {
        while matches!(self.peek_raw(), Some(Token::Semi) | Some(Token::Newline)) {
            self.advance_raw();
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, PulsarError> {
        // `let` is a statement-level construct, not an operator.
        if matches!(self.peek(), Some(Token::Let)) {
            return self.parse_let();
        }
        self.parse_pratt(0)
    }

    fn parse_let(&mut self) -> Result<Expr, PulsarError> {
        self.advance(); // consume `let`
        let name = match self.advance() {
            Some(Token::Ident(n)) => n.clone(),
            _ => return Err(self.error("expected identifier after `let`")),
        };
        self.expect(&Token::Eq)?;
        let value = self.parse_pratt(0)?;
        Ok(Expr::Let { name, value: Box::new(value) })
    }

    // ── Pratt parser ──────────────────────────────────────────────────────────

    fn parse_pratt(&mut self, min_bp: u8) -> Result<Expr, PulsarError> {
        let mut lhs = self.parse_prefix()?;

        loop {
            let Some(op) = self.peek_binop() else { break };
            let (l_bp, r_bp) = infix_binding_power(&op);
            if l_bp < min_bp {
                break;
            }
            self.advance(); // consume operator
            let rhs = self.parse_pratt(r_bp)?;
            lhs = Expr::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Expr, PulsarError> {
        match self.peek() {
            Some(Token::Bang) => {
                self.advance();
                let operand = self.parse_pratt(prefix_binding_power())?;
                Ok(Expr::UnaryOp { op: UnaryOp::Not, operand: Box::new(operand) })
            }
            Some(Token::Minus) => {
                self.advance();
                let operand = self.parse_pratt(prefix_binding_power())?;
                Ok(Expr::UnaryOp { op: UnaryOp::Neg, operand: Box::new(operand) })
            }
            _ => self.parse_atom(),
        }
    }

    fn parse_atom(&mut self) -> Result<Expr, PulsarError> {
        match self.peek() {
            Some(Token::Int(_)) => {
                if let Some(Token::Int(n)) = self.advance().cloned().as_ref().map(|t| t.clone()) {
                    Ok(Expr::Int(n))
                } else {
                    unreachable!()
                }
            }
            Some(Token::Float(_)) => {
                if let Token::Float(f) = self.advance().unwrap().clone() {
                    Ok(Expr::Float(f))
                } else {
                    unreachable!()
                }
            }
            Some(Token::True) => { self.advance(); Ok(Expr::Bool(true)) }
            Some(Token::False) => { self.advance(); Ok(Expr::Bool(false)) }
            Some(Token::Str(_)) => {
                if let Token::Str(s) = self.advance().unwrap().clone() {
                    Ok(Expr::Str(s))
                } else {
                    unreachable!()
                }
            }
            Some(Token::Ident(_)) => {
                if let Token::Ident(name) = self.advance().unwrap().clone() {
                    Ok(Expr::Var(name))
                } else {
                    unreachable!()
                }
            }
            Some(Token::LParen) => {
                self.advance();
                if matches!(self.peek(), Some(Token::RParen)) {
                    self.advance();
                    return Ok(Expr::Unit);
                }
                let expr = self.parse_pratt(0)?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            Some(Token::LBrace) => self.parse_block(),
            Some(Token::If) => self.parse_if(),
            None => Err(self.incomplete()),
            _ => Err(self.error("unexpected token")),
        }
    }

    fn parse_block(&mut self) -> Result<Expr, PulsarError> {
        self.expect(&Token::LBrace)?;
        let body = self.parse_block_body(/*top_level=*/false)?;
        self.expect(&Token::RBrace)?;
        Ok(match body {
            Expr::Block(_) => body,
            other => Expr::Block(vec![other]),
        })
    }

    fn parse_if(&mut self) -> Result<Expr, PulsarError> {
        self.advance(); // consume `if`
        let cond = self.parse_pratt(0)?;
        let then = self.parse_block()?;
        match self.peek() {
            Some(Token::Else) => {
                self.advance();
                let else_ = if matches!(self.peek(), Some(Token::If)) {
                    self.parse_if()?
                } else {
                    self.parse_block()?
                };
                Ok(Expr::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    else_: Box::new(else_),
                })
            }
            _ => Err(self.error("`if` requires an `else` branch")),
        }
    }

    /// Return the `BinOp` for the current token if it is an infix operator.
    fn peek_binop(&self) -> Option<BinOp> {
        match self.peek()? {
            Token::Plus     => Some(BinOp::Add),
            Token::Minus    => Some(BinOp::Sub),
            Token::Star     => Some(BinOp::Mul),
            Token::Slash    => Some(BinOp::Div),
            Token::Percent  => Some(BinOp::Rem),
            Token::EqEq     => Some(BinOp::Eq),
            Token::BangEq   => Some(BinOp::Ne),
            Token::Lt       => Some(BinOp::Lt),
            Token::LtEq     => Some(BinOp::Le),
            Token::Gt       => Some(BinOp::Gt),
            Token::GtEq     => Some(BinOp::Ge),
            Token::AmpAmp   => Some(BinOp::And),
            Token::PipePipe => Some(BinOp::Or),
            _ => None,
        }
    }
}

// ── Binding powers ────────────────────────────────────────────────────────────

/// Returns `(left_bp, right_bp)` for an infix operator.
/// Higher binding power = tighter binding.
fn infix_binding_power(op: &BinOp) -> (u8, u8) {
    match op {
        BinOp::Or  => (1, 2),
        BinOp::And => (3, 4),
        BinOp::Eq | BinOp::Ne => (5, 6),
        BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => (7, 8),
        BinOp::Add | BinOp::Sub => (9, 10),
        BinOp::Mul | BinOp::Div | BinOp::Rem => (11, 12),
    }
}

/// Binding power for prefix unary operators.
fn prefix_binding_power() -> u8 {
    13
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::BinOp;

    fn p(input: &str) -> Expr {
        parse(input).expect("parse failed")
    }

    #[test]
    fn parse_int_literal() {
        assert_eq!(p("42"), Expr::Int(42));
    }

    #[test]
    fn parse_float_literal() {
        assert_eq!(p("3.14"), Expr::Float(3.14));
    }

    #[test]
    fn parse_bool_literals() {
        assert_eq!(p("true"), Expr::Bool(true));
        assert_eq!(p("false"), Expr::Bool(false));
    }

    #[test]
    fn parse_string_literal() {
        assert_eq!(p(r#""hello""#), Expr::Str("hello".into()));
    }

    #[test]
    fn parse_unit() {
        assert_eq!(p("()"), Expr::Unit);
    }

    #[test]
    fn parse_variable() {
        assert_eq!(p("x"), Expr::Var("x".into()));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(
            p("1 + 2"),
            Expr::BinOp {
                op: BinOp::Add,
                lhs: Box::new(Expr::Int(1)),
                rhs: Box::new(Expr::Int(2)),
            }
        );
    }

    #[test]
    fn parse_precedence_mul_over_add() {
        // 1 + 2 * 3  →  1 + (2 * 3)
        assert_eq!(
            p("1 + 2 * 3"),
            Expr::BinOp {
                op: BinOp::Add,
                lhs: Box::new(Expr::Int(1)),
                rhs: Box::new(Expr::BinOp {
                    op: BinOp::Mul,
                    lhs: Box::new(Expr::Int(2)),
                    rhs: Box::new(Expr::Int(3)),
                }),
            }
        );
    }

    #[test]
    fn parse_parens_override_precedence() {
        // (1 + 2) * 3
        assert_eq!(
            p("(1 + 2) * 3"),
            Expr::BinOp {
                op: BinOp::Mul,
                lhs: Box::new(Expr::BinOp {
                    op: BinOp::Add,
                    lhs: Box::new(Expr::Int(1)),
                    rhs: Box::new(Expr::Int(2)),
                }),
                rhs: Box::new(Expr::Int(3)),
            }
        );
    }

    #[test]
    fn parse_unary_neg() {
        assert_eq!(
            p("-x"),
            Expr::UnaryOp {
                op: UnaryOp::Neg,
                operand: Box::new(Expr::Var("x".into())),
            }
        );
    }

    #[test]
    fn parse_unary_not() {
        assert_eq!(
            p("!true"),
            Expr::UnaryOp {
                op: UnaryOp::Not,
                operand: Box::new(Expr::Bool(true)),
            }
        );
    }

    #[test]
    fn parse_let_binding() {
        assert_eq!(
            p("let x = 42"),
            Expr::Let {
                name: "x".into(),
                value: Box::new(Expr::Int(42)),
            }
        );
    }

    #[test]
    fn parse_if_else() {
        assert_eq!(
            p("if true { 1 } else { 2 }"),
            Expr::If {
                cond: Box::new(Expr::Bool(true)),
                then: Box::new(Expr::Block(vec![Expr::Int(1)])),
                else_: Box::new(Expr::Block(vec![Expr::Int(2)])),
            }
        );
    }

    #[test]
    fn parse_block() {
        assert_eq!(
            p("{ 1; 2 }"),
            Expr::Block(vec![Expr::Int(1), Expr::Int(2)])
        );
    }

    #[test]
    fn parse_comparison_chain_left_assoc() {
        // 1 == 1 should parse as a BinOp, not produce an error
        assert!(matches!(p("1 == 1"), Expr::BinOp { op: BinOp::Eq, .. }));
    }

    #[test]
    fn incomplete_input_returns_sentinel() {
        let err = parse("1 +").unwrap_err();
        assert!(matches!(&err, PulsarError::ParseError { message, .. }
            if message.contains("incomplete")));
    }

    #[test]
    fn unknown_char_is_parse_error() {
        assert!(parse("@").is_err());
    }
}
