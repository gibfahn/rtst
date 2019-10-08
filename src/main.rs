fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        assert!(true);
    }

    #[test]
    fn test_hello_again() {
        assert!(!false);
    }
}
