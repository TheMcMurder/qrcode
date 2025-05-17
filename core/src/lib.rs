pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn hello() -> String {
    "Hello from Rust!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_eq!(hello(), "Hello from Rust!");
    }
}
