#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    If,
    Then,
    Else,
    Try,
    Catch
}
