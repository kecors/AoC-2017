use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter::FromIterator;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "dance.pest"]
struct DanceParser;

#[derive(Debug)]
enum Step {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

#[derive(Debug, Default)]
struct State {
    line: Vec<char>,
}

impl State {
    fn new() -> State {
        let line = "abcdefghijklmnop".chars().collect();
        State { line }
    }

    fn dance(&mut self, step: &Step) {
        match step {
            &Step::Spin(size) => {
                let mut tail: Vec<char> = self.line.split_off(16 - size);
                tail.extend(self.line.iter());
                self.line = tail;
            }
            &Step::Exchange(position1, position2) => {
                self.line.swap(position1, position2);
            }
            &Step::Partner(program1, program2) => {
                let position1 = self.line.iter().position(|&x| x == program1).unwrap();
                let position2 = self.line.iter().position(|&x| x == program2).unwrap();
                self.line.swap(position1, position2);
            }
        }
    }
}

fn parse_line(line: &str) -> Vec<Step> {
    let pairs = DanceParser::parse_str(Rule::dance, line).unwrap_or_else(|e| panic!("{}", e));

    let mut steps: Vec<Step> = Vec::new();

    let mut position1: usize = 0;
    let mut program1: char = '*';

    for pair in pairs {
        let rule = pair.as_rule();
        let text = pair.clone().into_span().as_str().to_string();
        match rule {
            Rule::size => {
                let size: usize = text.parse().unwrap();
                steps.push(Step::Spin(size));
            }
            Rule::position1 => {
                position1 = text.parse().unwrap();
            }
            Rule::position2 => {
                let position2: usize = text.parse().unwrap();
                steps.push(Step::Exchange(position1, position2));
            }
            Rule::program1 => {
                program1 = text.chars().next().unwrap();
            }
            Rule::program2 => {
                let program2 = text.chars().next().unwrap();
                steps.push(Step::Partner(program1, program2));
            }
            _ => {
                unimplemented!("parse_line");
            }
        }
    }

    steps
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let steps: Vec<Step> = parse_line(input.trim());
    //    println!("steps = {:?}", steps);

    // I found an internet discussion of Landau's function, which
    // establishes that a cycle is inevitable.

    // Use this to identify the cycle length
    let mut transforms: HashMap<Vec<char>, Vec<char>> = HashMap::new();
    // Track all permutations in order; index into this once
    // the offset is known
    let mut permutations: Vec<Vec<char>> = Vec::new();
    // Part 1 is concerned with the first run only
    let mut first_run: bool = true;

    let mut state = State::new();
    loop {
        let line_before = state.line.clone();
        for step in steps.iter() {
            state.dance(step);
        }
        let line_after = state.line.clone();
        let transforms_length: usize = transforms.len();
        match transforms.entry(line_before) {
            Entry::Vacant(v) => {
                permutations.push(line_after.clone());
                v.insert(line_after);
            }
            Entry::Occupied(_) => {
                let offset = 1000000000 % transforms_length;
                //println!("1000000000 % {} = {}", transforms_length, offset);
                println!("part 2: [{}]", String::from_iter(&permutations[offset - 1]));
                break;
            }
        }
        if first_run == true {
            println!("part 1: [{}]", String::from_iter(&state.line));
            first_run = false;
        }
    }
}
