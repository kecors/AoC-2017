use instruction::Instruction;

use pest::Parser;

#[derive(Parser)]
#[grammar = "instruction.pest"]
struct InstructionParser;

pub fn parse_line(line: &str) -> Instruction {
    let pairs = InstructionParser::parse_str(Rule::line, line).unwrap_or_else(|e| panic!("{}", e));

    let mut instruction: Instruction = Instruction::NoOp;

    for pair in pairs {
        let rule = pair.as_rule();
        let text = pair.clone().into_span().as_str().to_string();
        match rule {
            Rule::snd_r => {
                let register: char = text.chars().next().unwrap();
                instruction = Instruction::SndR(register);
            }
            Rule::snd_n => {
                let number: i64 = text.parse().unwrap();
                instruction = Instruction::SndN(number);
            }
            Rule::set_rr => {
                let mut iter = text.split_whitespace();
                let register1 = iter.next().unwrap().chars().next().unwrap();
                let register2 = iter.next().unwrap().chars().next().unwrap();
                instruction = Instruction::SetR(register1, register2);
            }
            Rule::set_rn => {
                let mut iter = text.split_whitespace();
                let register = iter.next().unwrap().chars().next().unwrap();
                let number = iter.next().unwrap().parse().unwrap();
                instruction = Instruction::SetN(register, number);
            }
            Rule::add_rr => {
                let mut iter = text.split_whitespace();
                let register1 = iter.next().unwrap().chars().next().unwrap();
                let register2 = iter.next().unwrap().chars().next().unwrap();
                instruction = Instruction::AddR(register1, register2);
            }
            Rule::add_rn => {
                let mut iter = text.split_whitespace();
                let register = iter.next().unwrap().chars().next().unwrap();
                let number = iter.next().unwrap().parse().unwrap();
                instruction = Instruction::AddN(register, number);
            }
            Rule::mul_rr => {
                let mut iter = text.split_whitespace();
                let register1 = iter.next().unwrap().chars().next().unwrap();
                let register2 = iter.next().unwrap().chars().next().unwrap();
                instruction = Instruction::MulR(register1, register2);
            }
            Rule::mul_rn => {
                let mut iter = text.split_whitespace();
                let register = iter.next().unwrap().chars().next().unwrap();
                let number = iter.next().unwrap().parse().unwrap();
                instruction = Instruction::MulN(register, number);
            }
            Rule::mod_rr => {
                let mut iter = text.split_whitespace();
                let register1 = iter.next().unwrap().chars().next().unwrap();
                let register2 = iter.next().unwrap().chars().next().unwrap();
                instruction = Instruction::ModR(register1, register2);
            }
            Rule::mod_rn => {
                let mut iter = text.split_whitespace();
                let register = iter.next().unwrap().chars().next().unwrap();
                let number = iter.next().unwrap().parse().unwrap();
                instruction = Instruction::ModN(register, number);
            }
            Rule::rcv_r => {
                let register: char = text.chars().next().unwrap();
                instruction = Instruction::RcvR(register);
            }
            Rule::rcv_n => {
                let number: i64 = text.parse().unwrap();
                instruction = Instruction::RcvN(number);
            }
            Rule::jgz_rr => {
                let mut iter = text.split_whitespace();
                let register1 = iter.next().unwrap().chars().next().unwrap();
                let register2 = iter.next().unwrap().chars().next().unwrap();
                instruction = Instruction::JgzRR(register1, register2);
            }
            Rule::jgz_rn => {
                let mut iter = text.split_whitespace();
                let register = iter.next().unwrap().chars().next().unwrap();
                let number = iter.next().unwrap().parse().unwrap();
                instruction = Instruction::JgzRN(register, number);
            }
            Rule::jgz_nr => {
                let mut iter = text.split_whitespace();
                let number = iter.next().unwrap().parse().unwrap();
                let register = iter.next().unwrap().chars().next().unwrap();
                instruction = Instruction::JgzNR(number, register);
            }
            Rule::jgz_nn => {
                let mut iter = text.split_whitespace();
                let number1 = iter.next().unwrap().parse().unwrap();
                let number2 = iter.next().unwrap().parse().unwrap();
                instruction = Instruction::JgzNN(number1, number2);
            }
            _ => {
                unimplemented!("parse_line");
            }
        }
    }

    instruction
}
