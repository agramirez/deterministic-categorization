fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_run() {
        assert!(true);
    }

    #[test]
    fn can_create_and_use_regex() {
        let re = categorizer::make_regex_action(r"(?im)\b(corn)\b",1.0);
        let expected = categorizer::CategoryMatch { yes: true, confidence: categorizer::Confidence { min: 1.0, max: 1.0, actual: 1.0} };
        let actual = re("let me know about some corn, please");

        assert_eq!(expected,actual,"expected left, but actual right is not the same");
    }

    #[test]
    fn is_better_yes() {
        let lesser = categorizer::CategoryMatch { yes: true, confidence: categorizer::Confidence { min: 0.1, max: 0.1, actual: 0.1 } };
        let greater = categorizer::CategoryMatch { yes: true, confidence: categorizer::Confidence { min: 0.2, max: 0.2, actual: 0.2 } };

        assert_eq!(true, lesser.is_other_better(&greater));
    }
}