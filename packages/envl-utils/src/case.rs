pub trait Case {
    fn gen(splited_txt: Vec<String>) -> String;
    fn parse(txt: &str) -> Vec<String>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnakeCase;

impl Case for SnakeCase {
    fn gen(splited_txt: Vec<String>) -> String {
        let mut result = String::new();

        for (i, txt) in splited_txt.iter().enumerate() {
            if i != 0 {
                result.push('_');
            }
            for c in txt.chars() {
                result.push_str(&c.to_lowercase().to_string());
            }
        }

        result
    }

    fn parse(txt: &str) -> Vec<String> {
        txt.split("_").map(|e| e.to_string()).collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CamelCase;

impl Case for CamelCase {
    fn gen(splited_txt: Vec<String>) -> String {
        let mut result = String::new();

        for (i, txt) in splited_txt.iter().enumerate() {
            for (txt_i, c) in txt.char_indices() {
                let c = if i != 0 && txt_i == 0 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_lowercase().to_string()
                };
                result.push_str(&c);
            }
        }

        result
    }

    fn parse(txt: &str) -> Vec<String> {
        let mut curr = String::new();
        let mut result = Vec::new();

        for c in txt.chars() {
            if c.is_uppercase() {
                result.push(curr.clone());
                curr.clear();
            }

            curr.push(c);
        }

        if !curr.is_empty() {
            result.push(curr.clone());
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::case::{CamelCase, Case, SnakeCase};

    #[test]
    fn snake_case_convert_test() {
        let result = SnakeCase::gen(CamelCase::parse("thisIsATest"));
        assert_eq!(result, "this_is_a_test");
    }

    #[test]
    fn camel_case_convert_test() {
        let result = CamelCase::gen(SnakeCase::parse("this_is_a_test"));
        assert_eq!(result, "thisIsATest");
    }
}
