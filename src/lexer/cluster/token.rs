#[derive(Debug, PartialEq)]
pub enum TopSet {
    Number(u32),
    Push,
    Pop,
}

#[derive(Debug, PartialEq)]
pub enum BottomSet {
    If,
    Print,
    Dup,
}
