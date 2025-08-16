#[macro_export]
macro_rules! envl_vars_error_message {
    ($msg: literal, $pos: ident) => {
        format!(
            "Error: {} (at {}:{}:{})",
            $msg, $pos.file_path, $pos.row, $pos.col
        )
    };
}

#[macro_export]
macro_rules! envl_vars_error {
    ($msg: literal, $pos: ident) => {
        use crate::envl_vars_error_message;

        let message = envl_vars_error_message!($msg, $pos);

        eprintln!("{}", message);
        std::process::exit(1);
    };
}
