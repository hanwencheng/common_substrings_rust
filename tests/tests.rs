#[cfg(test)]
mod tests {
    use common_substrings::{get_substrings};

    #[test]
    fn it_works() {
        let test_samples = vec![
            "java", "javascript", "typescript", "coffeescript", "coffee"
        ];

        let result_substrings = get_substrings(test_samples, 2, 3);
        result_substrings.iter().for_each(|it| {
            println!("{}", it);
        });
        assert!(true);
    }
}
