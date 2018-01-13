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
            },
            Rule::sub_rr => {
                let mut iter = text.split_whitespace();
                let register1 = iter.next().unwrap().chars().next().unwrap();
                let register2 = iter.next().unwrap().chars().next().unwrap();
                instruction = Instruction::SubR(register1, register2);
            }
            Rule::sub_rn => {
                let mut iter = text.split_whitespace();
                let register = iter.next().unwrap().chars().next().unwrap();
                let number = iter.next().unwrap().parse().unwrap();
                instruction = Instruction::SubN(register, number);
            },
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
            },
            Rule::jnz_rr => {
                let mut iter = text.split_whitespace();
                let register1 = iter.next().unwrap().chars().next().unwrap();
                let register2 = iter.next().unwrap().chars().next().unwrap();
                instruction = Instruction::JnzRR(register1, register2);
            },
            Rule::jnz_rn => {
                let mut iter = text.split_whitespace();
                let register = iter.next().unwrap().chars().next().unwrap();
                let number = iter.next().unwrap().parse().unwrap();
                instruction = Instruction::JnzRN(register, number);
            },
            Rule::jnz_nr => {
                let mut iter = text.split_whitespace();
                let number = iter.next().unwrap().parse().unwrap();
                let register = iter.next().unwrap().chars().next().unwrap();
                instruction = Instruction::JnzNR(number, register);
            },
            Rule::jnz_nn => {
                let mut iter = text.split_whitespace();
                let number1 = iter.next().unwrap().parse().unwrap();
                let number2 = iter.next().unwrap().parse().unwrap();
                instruction = Instruction::JnzNN(number1, number2);
            },
            _ => { unimplemented!("parse_line"); }
        }
    }

    instruction
}
