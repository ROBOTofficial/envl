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

    #[test]
    fn other_lang_codeblock_test() {
        let code = code_block! {
            export function a() {
                console.log("Hello World!!");
            }
        };

        assert_eq!(
            code.to_string(),
            "export function a () { console . log (\"Hello World!!\") ; }"
        );
    }
}
