use pulsar_core::{
    eval,
    parse,
    Session,
    value::EvalResult,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn banner() -> String {
    pulsar_core::banner::banner()
}

#[wasm_bindgen]
pub struct Repl {
    session: Session,
}

#[wasm_bindgen]
impl Repl {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { session: Session::new() }
    }

    pub fn eval(&mut self, input: &str) -> String {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return String::new();
        }
        self.session.push_history(trimmed);
        match parse(trimmed) {
            Err(e) => format!("error: {e}"),
            Ok(expr) => match eval(&expr, &mut self.session) {
                EvalResult::Ok { value, type_name } => format!("{value} : {type_name}"),
                EvalResult::Bound { name, value, type_name } => {
                    format!("let {name} = {value} : {type_name}")
                }
                EvalResult::Err(e) => format!("error: {e}"),
            },
        }
    }
}
