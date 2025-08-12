#[derive(Clone, Debug, PartialEq)]
pub enum Tokens {
    Number(f64),
    String(String),
    If,
    Then,
    Else,
    Try,
    Catch,
}
