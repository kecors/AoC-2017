use std::io::{stdin, Read};
use std::collections::HashMap;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "instruction.pest"]
struct InstructionParser;

// Binary Operator
#[derive(Debug)]
enum BinOp {
    INCREMENT,
    DECREMENT,
}

impl Default for BinOp {
    fn default() -> BinOp {
        BinOp::INCREMENT
    }
}

// Relational Operator
#[derive(Debug)]
enum RelOp {
    NOOP,
    EQ,
    NEQ,
    GTE,
    LTE,
    GT,
    LT,
}

impl Default for RelOp {
    fn default() -> RelOp {
        RelOp::NOOP
    }
}

#[derive(Debug, Default)]
struct Condition {
    register: String,
    relop: RelOp,
    operand: i32,
}

#[derive(Debug, Default)]
struct Instruction {
    register: String,
    binop: BinOp,
    operand: i32,
    cond: Condition,
}

#[derive(Debug)]
struct State {
    registers: HashMap<String, i32>,
    max_held: i32,
}

impl State {
    fn new(instructions: &Vec<Instruction>) -> State {
        let mut registers = HashMap::new();

        for instr in instructions {
            registers.entry(instr.register.clone()).or_insert(0);
        }

        State {
            registers: registers,
            max_held: 0,
        }
    }

    fn process_instruction(&mut self, instr: &Instruction) {
        if let Some(regval) = self.registers.get(&instr.cond.register) {
            let status: bool = match instr.cond.relop {
                RelOp::EQ => *regval == instr.cond.operand,
                RelOp::NEQ => *regval != instr.cond.operand,
                RelOp::GTE => *regval >= instr.cond.operand,
                RelOp::LTE => *regval <= instr.cond.operand,
                RelOp::GT => *regval > instr.cond.operand,
                RelOp::LT => *regval < instr.cond.operand,
                _ => true,
            };
            if !status {
                return;
            }
        }

        if let Some(regval) = self.registers.get_mut(&instr.register) {
            match instr.binop {
                BinOp::INCREMENT => {
                    *regval += instr.operand;
                }
                BinOp::DECREMENT => {
                    *regval -= instr.operand;
                }
            }
            if *regval > self.max_held {
                self.max_held = *regval;
            }
        }
    }

    fn report_maxes(&self) {
        let values: Vec<&i32> = self.registers.values().collect();
        println!("maximum value = {}", values.iter().max().unwrap());
        println!("max held = {}", self.max_held);
    }
}

fn parse_line(line: &str) -> Instruction {
    let mut instr = Instruction::default();

    let pairs = InstructionParser::parse_str(Rule::line, line).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        let rule = pair.as_rule();
        let text = pair.clone().into_span().as_str().to_string();
        match rule {
            Rule::register => {
                instr.register = text;
            }
            Rule::increment => {
                instr.binop = BinOp::INCREMENT;
            }
            Rule::decrement => {
                instr.binop = BinOp::DECREMENT;
            }
            Rule::operand => instr.operand = text.parse().unwrap(),
            Rule::condreg => {
                instr.cond.register = text;
            }
            Rule::eq => {
                instr.cond.relop = RelOp::EQ;
            }
            Rule::neq => {
                instr.cond.relop = RelOp::NEQ;
            }
            Rule::gte => {
                instr.cond.relop = RelOp::GTE;
            }
            Rule::lte => {
                instr.cond.relop = RelOp::LTE;
            }
            Rule::gt => {
                instr.cond.relop = RelOp::GT;
            }
            Rule::lt => {
                instr.cond.relop = RelOp::LT;
            }
            Rule::condop => instr.cond.operand = text.parse().unwrap(),
            _ => {
                println!("unknown rule {:?}", rule);
            }
        }
    }

    instr
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let instructions: Vec<Instruction> = input.lines().map(|line| parse_line(line)).collect();
    //println!("instructions = {:?}", instructions);

    let mut state = State::new(&instructions);
    //println!("state = {:?}", state);

    for instruction in instructions {
        //println!("instr = {:?}", instruction);
        state.process_instruction(&instruction);
        //println!("state = {:?}", state);
    }

    state.report_maxes();
}
