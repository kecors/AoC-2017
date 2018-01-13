extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::{stdin, Read};

mod instruction;
mod parsing;
mod coprocessor;

use instruction::Instruction;
use parsing::parse_line;
use coprocessor::Coprocessor;

fn do_part1(input: &String) {
    let instructions: Vec<Instruction> = input.lines().map(|line| parse_line(line)).collect();

    let mut coprocessor = Coprocessor::new(instructions);
    coprocessor.execute();
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    do_part1(&input);
}
