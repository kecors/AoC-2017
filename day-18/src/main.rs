extern crate either;

use std::io::{stdin, Read};
use either::*;
use std::collections::HashMap;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "instruction.pest"]
struct InstructionParser;

#[derive(Debug)]
enum Instruction {
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

fn parse_line(line: &str) -> Instruction {
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

#[derive(Debug)]
struct Duet {
    code_segment: Vec<Instruction>,
    instruction_pointer: i64,
    registers: HashMap<char, i64>,
    frequency: i64
}

impl Duet {
    fn new(instructions: Vec<Instruction>) -> Duet {
        let mut registers: HashMap<char, i64> = HashMap::new();
        for instruction in instructions.iter() {
            match instruction {
                &Instruction::NoOp => {
                },
                &Instruction::SndR(register) => {
                    registers.entry(register).or_insert(0);
                }
                &Instruction::SndN(_) => {
                },
                &Instruction::SetR(register1, register2) => {
                    registers.entry(register1).or_insert(0);
                    registers.entry(register2).or_insert(0);
                },
                &Instruction::SetN(register, _) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::AddR(register1, register2) => {
                    registers.entry(register1).or_insert(0);
                    registers.entry(register2).or_insert(0);
                },
                &Instruction::AddN(register, _) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::MulR(register1, register2) => {
                    registers.entry(register1).or_insert(0);
                    registers.entry(register2).or_insert(0);
                },
                &Instruction::MulN(register, _) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::ModR(register1, register2) => {
                    registers.entry(register1).or_insert(0);
                    registers.entry(register2).or_insert(0);
                },
                &Instruction::ModN(register, _) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::RcvR(register) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::RcvN(_) => {
                },
                &Instruction::JgzRR(register1, register2) => {
                    registers.entry(register1).or_insert(0);
                    registers.entry(register2).or_insert(0);
                },
                &Instruction::JgzRN(register, _) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::JgzNR(_, register) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::JgzNN(_, _) => {
                }
            }
        }

        Duet {
            code_segment: instructions,
            instruction_pointer: 0,
            registers: registers,
            frequency: 0
        }
    }

    fn execute(&mut self) {
        let mut jumped: bool = false;
        loop {
            match self.code_segment[self.instruction_pointer as usize] {
                Instruction::NoOp => {
                },
                Instruction::SndR(register) => {
                    if let Some(value) = self.registers.get(&register) {
                        self.frequency = *value;
                    };
                },
                Instruction::SndN(number) => {
                    self.frequency = number;
                },
                Instruction::SetR(register1, register2) => {
                    let mut new_value = 0;
                    if let Some(value) = self.registers.get(&register2) {
                        new_value = *value;
                    }
                    if let Some(value) = self.registers.get_mut(&register1) {
                        *value = new_value;
                    }
                },
                Instruction::SetN(register, number) => {
                    if let Some(value) = self.registers.get_mut(&register) {
                        *value = number;
                    };
                },
                Instruction::AddR(register1, register2) => {
                    let mut addend = 0;
                    if let Some(value) = self.registers.get(&register2) {
                        addend = *value;
                    }
                    if let Some(value) = self.registers.get_mut(&register1) {
                        *value += addend;
                    }
                },
                Instruction::AddN(register, number) => {
                    if let Some(value) = self.registers.get_mut(&register) {
                        *value += number;
                    }
                },
                Instruction::MulR(register1, register2) => {
                    let mut factor = 0;
                    if let Some(value) = self.registers.get(&register2) {
                        factor = *value;
                    }
                    if let Some(value) = self.registers.get_mut(&register1) {
                        *value *= factor;
                    }
                },
                Instruction::MulN(register, number) => {
                    if let Some(value) = self.registers.get_mut(&register) {
                        *value *= number;
                    }
                },
                Instruction::ModR(register1, register2) => {
                    let mut divisor = 0;
                    if let Some(value) = self.registers.get(&register2) {
                        divisor = *value;
                    }
                    if let Some(value) = self.registers.get_mut(&register1) {
                        *value = *value % divisor;
                    }
                },
                Instruction::ModN(register, number) => {
                    if let Some(value) = self.registers.get_mut(&register) {
                        *value = *value % number;
                    }
                },
                Instruction::RcvR(register) => {
                    if let Some(value) = self.registers.get(&register) {
                        if *value != 0 {
                            println!("part 1: frequency = {}", self.frequency);
                            return;
                        }
                    }
                },
                Instruction::RcvN(number) => {
                    if number != 0 {
                        println!("part 1: frequency = {}", self.frequency);
                        return;
                    }
                },
                Instruction::JgzRR(register1, register2) => {
                    let mut x = 0;
                    if let Some(value) = self.registers.get(&register1) {
                        x = *value;
                    }
                    if let Some(offset) = self.registers.get(&register2) {
                        if x > 0 {
                            self.instruction_pointer += *offset;
                            jumped = true;
                        }
                    }
                },
                Instruction::JgzRN(register, number) => {
                    let mut x = 0;
                    if let Some(value) = self.registers.get(&register) {
                        x = *value;
                    }
                    if x > 0 {
                        self.instruction_pointer += number;
                        jumped = true;
                    }
                },
                Instruction::JgzNR(number, register) => {
                    if let Some(offset) = self.registers.get(&register) {
                        if number > 0 {
                            self.instruction_pointer += *offset;
                            jumped = true;
                        }
                    }
                },
                Instruction::JgzNN(number1, number2) => {
                    if number1 > 0 {
                        self.instruction_pointer += number2;
                        jumped = true;
                    }
                }
            }
            if jumped == false {
                self.instruction_pointer += 1;
            }
            jumped = false;
            // Since the puzzle presumes that rcv will be executed, ignore
            // the possibility of continuing or jumping off either end
            // of the progam.
        }
    }
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();
//    println!("input = {:#?}", input);

    let instructions: Vec<Instruction> = input.lines().map(|line| parse_line(line)).collect();
//    println!("instructions = {:?}", instructions);

    let mut duet = Duet::new(instructions);
//    println!("duet = {:?}", duet);

    duet.execute();
}
