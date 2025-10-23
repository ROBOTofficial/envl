#[cfg(test)]
mod test {
    use envl_codeblock::code_block;

    #[test]
    fn normal_codeblock_test() {
        let code = code_block! {
            pub fn a() {}
        };
        assert_eq!(code.to_string(), "pub fn a () { }");
    }
}
