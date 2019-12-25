#[cfg(test)]
mod tests {
    use common_substrings_rust::*;

    #[test]
    fn it_works() {
        let number_vector = vec![1, 2, 3];
        let string_vector = vec!["java", "javascript", "pythonscript"];
        const STRING_ARRAY:[&str; 3] = ["java", "javascript", "pythonsscript"];

        build_array(string_vector);
        assert_eq!(add(2 , 2), 4);
    }
}
