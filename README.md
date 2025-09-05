# Enviroment Language (envl)

## Packages

|language|lib                                  |repository                               |
|--------|-------------------------------------|-----------------------------------------|
|Rust    |[envl](https://crates.io/crates/envl)|[ROBOTofficial/envl](./packages/envl-rs/)|

## Cli

|name    |repository                                |
|--------|------------------------------------------|
|envl-cli|[ROBOTofficial/envl](./packages/envl-cli/)|

## Examples

**.envl**
```rs
a = "123";
b = 123;
c = true;
d = [123, 456];
```

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
