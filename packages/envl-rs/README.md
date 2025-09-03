# Envl for Rust

This is an envl lib for Rust.

## Usage

For more details, please see [here](../../tests/envl-rs-test).

**.envlconf**
```rs
settings {}

vars {
    a: string,
    b: int,
    c: bool,
    d: Array<int>
}
```

**.envl**
```rs
a = "123";
b = 123;
c = true;
d = [123, 456];
```

**Cargo.toml**
```rs
[package]
...
build = "build.rs"

[dependencies]
envl = { version = "0.4.0" }

[build-dependencies]
envl = { version = "0.4.0" }
```

**build.rs**
```rs
use envl::load_envl;

fn main() {
    if let Err(err) = load_envl("src/envl.rs".to_string()) {
        panic!("{:?}", err);
    };
}
```

**src/main.rs**
```rs
pub mod envl;

pub fn main() {
    let env = envl::envl();

    println!("{}", env.a);
    println!("{}", env.b);
    println!("{}", env.c);
    println!("{:?}", env.d);
}
```
