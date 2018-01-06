use std::io;

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
    Partner(char, char)
}

#[derive(Debug, Default)]
struct State {
    line: Vec<char>
}

impl State {
    fn new() -> State {
        let line = "abcdefghijklmnop".chars().collect();
        State {
            line
        }
    }

    fn dance(&mut self, step: Step) {
        match step {
            Step::Spin(size) => {
                let mut tail: Vec<char> = self.line.split_off(16 - size);
                tail.extend(self.line.iter());
                self.line = tail;
            },
            Step::Exchange(position1, position2) => {
                self.line.swap(position1, position2);
            },
            Step::Partner(program1, program2) => {
                let position1 = self.line.iter()
                                         .position(|&x| x == program1)
                                         .unwrap();
                let position2 = self.line.iter()
                                         .position(|&x| x == program2)
                                         .unwrap();
                self.line.swap(position1, position2);
            }
        }
    }

    fn display(&mut self) {
        print!("part1: [");
        for program in self.line.iter() {
            print!("{}", program);
        }
        println!("]");
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
            },
            Rule::position1 => {
                position1 = text.parse().unwrap();
            },
            Rule::position2 => {
                let position2: usize = text.parse().unwrap();
                steps.push(Step::Exchange(position1, position2));
            },
            Rule::program1 => {
                program1 = text.chars().next().unwrap();
            },
            Rule::program2 => {
                let program2 = text.chars().next().unwrap();
                steps.push(Step::Partner(program1, program2));
            },
            _ => { unimplemented!("parse_line"); }
        }
    }

    steps
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let steps: Vec<Step> = parse_line(input.trim());
//    println!("steps = {:?}", steps);

    let mut state = State::new();

    for step in steps {
        state.dance(step);
    }

    state.display();
}
