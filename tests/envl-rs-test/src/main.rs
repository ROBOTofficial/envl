pub mod envl;

pub fn main() {
    let env = envl::envl();

    println!("{}", env.a);
    println!("{}", env.b);
    println!("{}", env.c);
    println!("{:?}", env.d);
}

#[cfg(test)]
mod test {
    use crate::envl::{envl, StructStructev};

    #[test]
    fn env_check() {
        let v = StructStructev {
            a: String::from("hello world"),
        };

        let env = envl();
        assert_eq!(env.a, "123".to_string());
        assert_eq!(env.b, 123);
        assert!(env.c);
        assert_eq!(env.d, vec![123, 456]);
        assert_eq!(env.e.v, v);
        assert_eq!(env.e.x, 111);
        assert!(!env.e.y);
        assert_eq!(env.e.z, vec!["hello".to_string(), "world".to_string()]);
    }
}
