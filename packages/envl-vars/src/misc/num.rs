pub fn is_num(input: String, sign: bool) -> bool {
    let mut signed = false;
    for c in input.chars() {
        if sign && !signed && c == '-' {
            signed = true;
            continue;
        }
        if !c.is_ascii_digit() {
            return false;
        }
    }

    true
}
