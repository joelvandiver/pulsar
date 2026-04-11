use std::collections::HashMap;

use crate::value::{EvalResult, Value};

// ── Session ───────────────────────────────────────────────────────────────────

/// Persistent state for a single Pulsar REPL session.
///
/// `Session` is the single owner of all mutable state that survives between
/// REPL inputs: variable bindings, input history, and optional snapshots.
/// Pass it by `&mut` to [`crate::eval`] on every input.
#[derive(Debug, Default, Clone)]
pub struct Session {
    /// Variable bindings installed by `let` expressions.
    pub bindings: HashMap<String, Value>,

    /// Raw input strings, in the order they were submitted.
    pub history: Vec<String>,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a raw input string into history.
    ///
    /// Called by the REPL frontend after every successful submission.
    pub fn push_history(&mut self, input: impl Into<String>) {
        self.history.push(input.into());
    }

    /// Returns a snapshot of the current session state.
    ///
    /// Snapshots are cheap clones — useful for step-through evaluation and
    /// for undoing a bad input.
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            bindings: self.bindings.clone(),
            history_len: self.history.len(),
        }
    }

    /// Restore the session to a previously taken [`Snapshot`].
    ///
    /// Bindings are replaced wholesale; history is truncated to the length
    /// recorded in the snapshot.
    pub fn restore(&mut self, snapshot: &Snapshot) {
        self.bindings = snapshot.bindings.clone();
        self.history.truncate(snapshot.history_len);
    }

    /// Re-evaluate all inputs in `history` into a fresh session, returning
    /// the results in order.
    ///
    /// Useful for reconstructing session state after a restart, or for
    /// exporting a session as a script.
    pub fn replay(&self) -> Vec<(String, EvalResult)> {
        use crate::{eval::eval, parser::parse};

        let mut fresh = Session::new();
        self.history
            .iter()
            .map(|input| {
                let result = match parse(input) {
                    Ok(expr) => eval(&expr, &mut fresh),
                    Err(e) => EvalResult::Err(e),
                };
                (input.clone(), result)
            })
            .collect()
    }
}

// ── Snapshot ──────────────────────────────────────────────────────────────────

/// A point-in-time copy of a [`Session`]'s mutable state.
///
/// Created by [`Session::snapshot`]; applied back with [`Session::restore`].
#[derive(Debug, Clone)]
pub struct Snapshot {
    bindings: HashMap<String, Value>,
    history_len: usize,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{eval::eval, parser::parse, value::Value};

    fn eval_in(session: &mut Session, input: &str) -> EvalResult {
        let expr = parse(input).expect("parse failed");
        let result = eval(&expr, session);
        session.push_history(input);
        result
    }

    #[test]
    fn history_records_inputs() {
        let mut s = Session::new();
        eval_in(&mut s, "let x = 1");
        eval_in(&mut s, "let y = 2");
        assert_eq!(s.history, vec!["let x = 1", "let y = 2"]);
    }

    #[test]
    fn snapshot_restore_reverts_bindings() {
        let mut s = Session::new();
        eval_in(&mut s, "let x = 10");

        let snap = s.snapshot();

        eval_in(&mut s, "let x = 99");
        assert_eq!(s.bindings.get("x"), Some(&Value::Int(99)));

        s.restore(&snap);
        assert_eq!(s.bindings.get("x"), Some(&Value::Int(10)));
    }

    #[test]
    fn snapshot_restore_truncates_history() {
        let mut s = Session::new();
        eval_in(&mut s, "let a = 1");
        let snap = s.snapshot();
        eval_in(&mut s, "let b = 2");

        assert_eq!(s.history.len(), 2);
        s.restore(&snap);
        assert_eq!(s.history.len(), 1);
    }

    #[test]
    fn replay_reconstructs_bindings() {
        let mut s = Session::new();
        eval_in(&mut s, "let x = 6");
        eval_in(&mut s, "let y = x * 7");

        let replayed = s.replay();
        assert_eq!(replayed.len(), 2);

        // Last replayed result should be y = 42
        assert!(matches!(
            &replayed[1].1,
            EvalResult::Bound { name, value, .. }
                if name == "y" && *value == Value::Int(42)
        ));
    }

    #[test]
    fn replay_is_independent_of_original_session() {
        let mut s = Session::new();
        eval_in(&mut s, "let x = 1");

        // Mutate the original after recording history
        s.bindings.insert("x".into(), Value::Int(999));

        // Replay should see x = 1, not 999
        let replayed = s.replay();
        assert!(matches!(
            &replayed[0].1,
            EvalResult::Bound { value, .. } if *value == Value::Int(1)
        ));
    }
}
