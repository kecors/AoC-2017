extern crate either;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::{stdin, Read};

mod instruction;
mod parsing;
mod duet;

use instruction::Instruction;
use parsing::parse_line;
use duet::Duet;

use std::thread;
use std::sync::mpsc;

fn do_part1(input: &String) {
    let instructions: Vec<Instruction> = input.lines().map(|line| parse_line(line)).collect();

    let (tx, rx) = mpsc::channel();

    let mut duet = Duet::new(instructions, 0, tx, rx, false);
    duet.execute();
}

fn do_part2(input: &String) {
    let instructions0: Vec<Instruction> = input.lines().map(|line| parse_line(line)).collect();

    let (tx0, rx0) = mpsc::channel();
    let (tx1, rx1) = mpsc::channel();

    let instructions1 = instructions0.clone();

    thread::spawn(move || {
        let mut duet0 = Duet::new(instructions0, 0, tx0, rx1, true);
        duet0.execute();
    });

    let mut duet1 = Duet::new(instructions1, 1, tx1, rx0, true);
    duet1.execute();
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    do_part1(&input);
    do_part2(&input);
}
