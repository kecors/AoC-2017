extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::{stdin, Read};

mod instruction;
mod parsing;
mod coprocessor;
mod part2;

use instruction::Instruction;
use parsing::parse_line;
use coprocessor::Coprocessor;

fn do_part1(input: &String) {
    let instructions: Vec<Instruction> = input.lines().map(|line| parse_line(line)).collect();

    let mut coprocessor = Coprocessor::new(instructions);
    coprocessor.execute(false);
}

fn do_part2(input: &String) {
    let instructions: Vec<Instruction> = input.lines().map(|line| parse_line(line)).collect();

    let mut coprocessor = Coprocessor::new(instructions);
    coprocessor.set_register('a', 1);
    coprocessor.execute(true);
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    do_part1(&input);

    // Naive brute force approach takes a very long time to finish.
    // Instead, translate the assembly code and make an optimization.
    let mut part2_state = part2::State::new();
    part2_state.run();
}
