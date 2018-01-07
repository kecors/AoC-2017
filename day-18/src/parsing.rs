use instruction::Instruction;
use either::{Either, Left, Right};

use pest::Parser;

#[derive(Parser)]
#[grammar = "instruction.pest"]
struct InstructionParser;

pub fn parse_line(line: &str) -> Instruction {
    let pairs = InstructionParser::parse_str(Rule::line, line).unwrap_or_else(|e| panic!("{}", e));

    let mut instruction: Instruction = Instruction::NoOp;

    let mut set1r: char = '*';
    let mut add1r: char = '*';
    let mut mul1r: char = '*';
    let mut mod1r: char = '*';
    let mut jgz1: Either<char, i64> = Left('*');

    for pair in pairs {
        let rule = pair.as_rule();
        let text = pair.clone().into_span().as_str().to_string();
        match rule {
            Rule::snd1r => {
                let register: char = text.chars().next().unwrap();
                instruction = Instruction::SndR(register);
            },
            Rule::snd1n => {
                let number: i64 = text.parse().unwrap();
                instruction = Instruction::SndN(number);
            },
            Rule::set1r => {
                set1r = text.chars().next().unwrap();
            },
            Rule::set2r => {
                let register: char = text.chars().next().unwrap();
                instruction = Instruction::SetR(set1r, register);
            }
            Rule::set2n => {
                let number: i64 = text.parse().unwrap();
                instruction = Instruction::SetN(set1r, number);
            },
            Rule::add1r => {
                add1r = text.chars().next().unwrap();
            },
            Rule::add2r => {
                let register: char = text.chars().next().unwrap();
                instruction = Instruction::AddR(add1r, register);
            }
            Rule::add2n => {
                let number: i64 = text.parse().unwrap();
                instruction = Instruction::AddN(add1r, number);
            },
            Rule::mul1r => {
                mul1r = text.chars().next().unwrap();
            },
            Rule::mul2r => {
                let register: char = text.chars().next().unwrap();
                instruction = Instruction::MulR(mul1r, register);
            }
            Rule::mul2n => {
                let number: i64 = text.parse().unwrap();
                instruction = Instruction::MulN(mul1r, number);
            },
            Rule::mod1r => {
                mod1r = text.chars().next().unwrap();
            },
            Rule::mod2r => {
                let register: char = text.chars().next().unwrap();
                instruction = Instruction::ModR(mod1r, register);
            }
            Rule::mod2n => {
                let number: i64 = text.parse().unwrap();
                instruction = Instruction::ModN(mod1r, number);
            },
            Rule::rcv1r => {
                let register: char = text.chars().next().unwrap();
                instruction = Instruction::RcvR(register);
            },
            Rule::rcv1n => {
                let number: i64 = text.parse().unwrap();
                instruction = Instruction::RcvN(number);
            },
            Rule::jgz1r => {
                jgz1 = Left(text.chars().next().unwrap());
            },
            Rule::jgz1n => {
                jgz1 = Right(text.parse().unwrap());
            },
            Rule::jgz2r => {
                let register: char = text.chars().next().unwrap();
                instruction = match jgz1 {
                    Left(jgz1r)  => Instruction::JgzRR(jgz1r, register),
                    Right(jgz1n) => Instruction::JgzNR(jgz1n, register)
                };
            }
            Rule::jgz2n => {
                let number: i64 = text.parse().unwrap();
                instruction = match jgz1 {
                    Left(jgz1r)  => Instruction::JgzRN(jgz1r, number),
                    Right(jgz1n) => Instruction::JgzNN(jgz1n, number)
                };
            },
            _ => { unimplemented!("parse_line"); }
        }
    }

    instruction
}
