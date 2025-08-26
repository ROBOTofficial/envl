#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub file_path: String,
    pub row: usize,
    pub col: usize,
}
