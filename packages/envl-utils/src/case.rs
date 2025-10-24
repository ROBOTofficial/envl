pub fn to_snake_case(txt: &str) -> String {
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

pub fn to_camel_case(txt: &str) -> String {
    let mut result = String::new();
    let mut is_target = false;

    for c in txt.chars() {
        if c == '_' {
            is_target = true;
        } else if is_target {
            result.push_str(&c.to_uppercase().to_string());
            is_target = false;
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use crate::case::{to_camel_case, to_snake_case};

    #[test]
    fn snake_case_convert_test() {
        assert_eq!(to_snake_case("thisIsATest"), "this_is_a_test");
    }

    #[test]
    fn camel_case_convert_test() {
        assert_eq!(to_camel_case("this_is_a_test"), "thisIsATest");
    }
}
