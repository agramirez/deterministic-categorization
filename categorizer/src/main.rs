fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod Test {
    use super::*;

    #[test]
    fn basic_run() {
        assert!(true);
    }
}