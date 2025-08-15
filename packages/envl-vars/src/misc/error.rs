#[macro_export]
macro_rules! envl_vars_error {
    ($msg: literal, $pos: ident) => {
        let message = format!(
            "Error: {} (at {}:{}:{})",
            $msg, $pos.file_path, $pos.row, $pos.col
        );
        eprintln!("{}", message);
        std::process::exit(1);
    };
}
