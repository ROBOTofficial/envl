#[macro_export]
macro_rules! envl_vars_error_message {
    ($msg: literal, $pos: ident) => {
        format!(
            "Error: {} (at {}:{}:{})",
            $msg, $pos.file_path, $pos.row, $pos.col
        )
    };
}
