pub fn is_num(input: String) -> bool {
    for c in input.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
    }

    true
}
