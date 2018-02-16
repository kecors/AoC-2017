#[derive(Debug, Clone)]
pub enum Instruction {
    NoOp,
    SetR(char, char),
    SetN(char, i64),
    SubR(char, char),
    SubN(char, i64),
    MulR(char, char),
    MulN(char, i64),
    JnzRR(char, char),
    JnzRN(char, i64),
    JnzNR(i64, char),
    JnzNN(i64, i64),
}
