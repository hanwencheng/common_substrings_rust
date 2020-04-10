#[cfg(test)]
mod tests {
    use common_substrings_rust::{get_substrings, Substring};

    #[test]
    fn it_works() {
        let test_samples = vec![
            "java",
            "offe",
            "coffescript",
            "typescript",
            "typed",
            "javacoffie",
            "fessss",
            "fe",
        ];

        let result_substrings = get_substrings(test_samples, 2, 3);
        result_substrings.iter().for_each(|it| {
            println!("{}", it);
        });
        assert!(true);
    }
}
