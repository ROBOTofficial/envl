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

    #[test]
    fn interpolation_test() {
        let value = code_block! {
            123
        };
        let code = code_block! {
            const fuga = #value;
        };

        assert_eq!(code.to_string(), "const fuga = 123 ;");
    }

    #[test]
    fn iter_test() {
        let values = vec![
            code_block! {
                hoge: 123
            },
            code_block! {
                huga: 456
            },
        ];
        let code = code_block! {
            struct Foo {
                #(#values),*
            }
        };

        assert_eq!(code.to_string(), "struct Foo { hoge : 123 , huga : 456 }");
    }
}
