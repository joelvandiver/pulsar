use logos::Logos;

/// A single token produced by the Pulsar lexer.
///
/// `logos` drives the tokenization; each variant's pattern is matched against
/// the input in source order (longest match wins).
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r]+")] // skip horizontal whitespace; newlines are significant for the REPL
pub enum Token {
    // --- Literals ---
    /// Floating-point literal — must be tried before `Int` so `3.14` doesn't
    /// lex as `Int(3)` followed by `.` followed by `Int(14)`.
    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),

    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Int(i64),

    #[token("true")]
    True,

    #[token("false")]
    False,

    /// String literal — captures the content between the double quotes.
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        // Strip surrounding quotes and unescape basic sequences.
        Some(unescape(&s[1..s.len() - 1]))
    })]
    Str(String),

    // --- Keywords ---
    #[token("let")]
    Let,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    // --- Identifiers (after keywords so keywords are matched first) ---
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    // --- Operators ---
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("==")]
    EqEq,

    #[token("!=")]
    BangEq,

    #[token("<=")]
    LtEq,

    #[token(">=")]
    GtEq,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

    #[token("&&")]
    AmpAmp,

    #[token("||")]
    PipePipe,

    #[token("!")]
    Bang,

    #[token("=")]
    Eq,

    // --- Delimiters ---
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token(";")]
    Semi,

    #[token("\n")]
    Newline,
}

fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some('\\') => out.push('\\'),
                Some('"') => out.push('"'),
                Some(other) => { out.push('\\'); out.push(other); }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Lex `input` into a `Vec` of `(Token, byte_range)` pairs.
///
/// Returns `Err` with the byte offset of the first unrecognised character.
pub fn lex(input: &str) -> Result<Vec<(Token, std::ops::Range<usize>)>, usize> {
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(input);
    while let Some(result) = lexer.next() {
        match result {
            Ok(tok) => tokens.push((tok, lexer.span())),
            Err(_) => return Err(lexer.span().start),
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens(input: &str) -> Vec<Token> {
        lex(input).unwrap().into_iter().map(|(t, _)| t).collect()
    }

    #[test]
    fn lex_integers() {
        assert_eq!(tokens("42"), vec![Token::Int(42)]);
        assert_eq!(tokens("-7"), vec![Token::Int(-7)]);
    }

    #[test]
    fn lex_floats() {
        assert_eq!(tokens("3.14"), vec![Token::Float(3.14)]);
    }

    #[test]
    fn lex_booleans() {
        assert_eq!(tokens("true false"), vec![Token::True, Token::False]);
    }

    #[test]
    fn lex_string() {
        assert_eq!(tokens(r#""hello""#), vec![Token::Str("hello".into())]);
    }

    #[test]
    fn lex_string_escape() {
        assert_eq!(tokens(r#""a\nb""#), vec![Token::Str("a\nb".into())]);
    }

    #[test]
    fn lex_keywords_vs_idents() {
        assert_eq!(tokens("let x"), vec![Token::Let, Token::Ident("x".into())]);
        assert_eq!(tokens("letter"), vec![Token::Ident("letter".into())]);
    }

    #[test]
    fn lex_operators() {
        assert_eq!(
            tokens("+ - * / % == != <= >= < > && || !"),
            vec![
                Token::Plus, Token::Minus, Token::Star, Token::Slash, Token::Percent,
                Token::EqEq, Token::BangEq, Token::LtEq, Token::GtEq,
                Token::Lt, Token::Gt, Token::AmpAmp, Token::PipePipe, Token::Bang,
            ]
        );
    }

    #[test]
    fn lex_error_on_unknown_char() {
        assert!(lex("@").is_err());
    }
}
