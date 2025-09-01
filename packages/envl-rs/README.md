# Envl for Rust

This is an envl lib for Rust.

**We have suspended the release due to bugs found in some systems.**
**We will do our utmost to restore this lib.**

## Usage
```rs
use envl::load_envl;

use crate::Env;

fn main() {
    let env = load_envl::<Env>();
}
```
