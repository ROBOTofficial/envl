use envl::load_envl;

fn main() {
    if let Err(err) = load_envl("src/envl.rs".to_string()) {
        panic!("{:?}", err);
    };
}
