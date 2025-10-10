use regex::RegexBuilder;

pub fn is_valid_variable_name(name: &str) -> bool {
    if let Ok(regex) = RegexBuilder::new(r"^[A-Za-z_]+$").build() {
        regex.is_match(name)
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use crate::name::is_valid_variable_name;

    #[test]
    fn variable_name_check() {
        assert!(is_valid_variable_name("abcdef"));
        assert!(is_valid_variable_name("abc_def"));
        assert!(is_valid_variable_name("AbCdEf"));
        assert!(is_valid_variable_name("AbC_dEf"));

        assert!(!is_valid_variable_name(""));
        assert!(!is_valid_variable_name("abc?def"));
        assert!(!is_valid_variable_name("abc[]"));
    }
}
