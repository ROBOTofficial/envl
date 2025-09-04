use crate::envl::envl;

pub mod envl;

pub fn main() {
    let env = envl();

    println!("{}", env.a);
    println!("{}", env.b);
    println!("{}", env.c);
    println!("{:?}", env.d);
}

#[cfg(test)]
mod test {
    use crate::envl::envl;

    #[test]
    fn env_check() {
        let env = envl();
        assert_eq!(env.a, "123".to_string());
        assert_eq!(env.b, 123);
        assert_eq!(env.c, true);
        assert_eq!(env.d, vec![123, 456]);
    }
}
