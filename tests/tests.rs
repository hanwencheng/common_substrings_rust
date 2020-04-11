#[cfg(test)]
mod tests {
    use common_substrings::{get_substrings};

    #[test]
    fn it_works() {
        let test_samples = vec![
            "science", "typescript", "crisis", "kept", "javascript", "java"
        ];

        let result_substrings = get_substrings(test_samples, 2, 4);
        result_substrings.iter().for_each(|it| {
            println!("{}", it);
        });
        assert_eq!(result_substrings.len(), 2);
    }
}
