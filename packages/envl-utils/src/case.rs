pub fn camel_case_to_snake_case(txt: &str) -> String {
    let mut result = String::new();

    for (i, c) in txt.char_indices() {
        let converted = if c.is_uppercase() {
            if i == 0 {
                c.to_lowercase().to_string()
            } else {
                format!("_{}", c.to_lowercase())
            }
        } else {
            c.to_string()
        };
        result.push_str(&converted);
    }

    result
}

#[cfg(test)]
mod test {
    use crate::case::camel_case_to_snake_case;

    #[test]
    fn convert_test() {
        assert_eq!(
            camel_case_to_snake_case("thisIsATest"),
            "this_is_a_test".to_string()
        );
    }
}
