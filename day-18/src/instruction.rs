#[derive(Debug, Clone)]
pub enum Instruction {
    NoOp,
    SndR(char),
    SndN(i64),
    SetR(char, char),
    SetN(char, i64),
    AddR(char, char),
    AddN(char, i64),
    MulR(char, char),
    MulN(char, i64),
    ModR(char, char),
    ModN(char, i64),
    RcvR(char),
    RcvN(i64),
    JgzRR(char, char),
    JgzRN(char, i64),
    JgzNR(i64, char),
    JgzNN(i64, i64)
}
