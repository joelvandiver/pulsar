pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

pub fn add(left: u64, right: u64) -> u64 {
    let test: Result<String, ()> = Ok("abc".to_string());
    let _ = test.unwrap();
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
