#[derive(Debug, PartialEq)]
pub enum TopSet {
    Number(i32),
    Negate,
    Push,
    Pop,
}

#[derive(Debug, PartialEq)]
pub enum BottomSet {
    If,
    Print,
    Input,
    Dup,
    Jump,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
    Bsl,
    Bsr,
    Equal,
    Greater,
    Less,
    Cycle,
}
