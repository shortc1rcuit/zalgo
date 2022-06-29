#[derive(Debug, PartialEq)]
pub enum TopSet {
    Number(i32),
    Push,
    Pop,
}

#[derive(Debug, PartialEq)]
pub enum BottomSet {
    If,
    Print,
    Input,
    Dup,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
