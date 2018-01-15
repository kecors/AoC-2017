use std::io::{stdin, Read};
use std::collections::{HashMap, HashSet};

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "blueprint.pest"]
struct BlueprintParser;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Left
    }
}

#[derive(Debug, Default, Clone)]
struct Rules {
    write: u8,
    direction: Direction,
    next_state: char,
}

#[derive(Debug, Default, Clone)]
struct State {
    id: char,
    on_zero: Rules,
    on_one: Rules,
}

#[derive(Debug, Default)]
struct Machine {
    step_limit: u64,
    step_count: u64,
    states: HashMap<char, State>,
    current_state: char,
    tape: HashSet<i64>,
    cursor: i64,
}

impl Machine {
    fn new() -> Machine {
        Machine::default()
    }

    fn parse(&mut self, lines: Vec<&str>) {
        let mut state: State = State::default();
        let mut state_zero_flag: bool = true;

        for line in lines {
            let pairs = BlueprintParser::parse_str(Rule::line, line).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                let rule = pair.as_rule();
                let text = pair.clone().into_span().as_str().to_string();
                match rule {
                    Rule::begin_state => {
                        self.current_state = text.chars().next().unwrap();
                    },
                    Rule::step_limit => {
                        self.step_limit = text.parse().unwrap();
                    },
                    Rule::state_id => {
                        state.id = text.chars().next().unwrap();
                    },
                    Rule::current_value => {
                        if text == "0" {
                            state_zero_flag = true;
                        } else {
                            state_zero_flag = false;
                        }
                    },
                    Rule::write_value => {
                        if state_zero_flag == true {
                            state.on_zero.write = text.parse().unwrap();
                        } else {
                            state.on_one.write = text.parse().unwrap();
                        }
                    },
                    Rule::move_direction => {
                        let direction = match text.as_str() {
                            "left"  => Direction::Left,
                            "right" => Direction::Right,
                            _       => unimplemented!("")
                        };
                        if state_zero_flag == true {
                            state.on_zero.direction = direction;
                        } else {
                            state.on_one.direction = direction;
                        }

                    },
                    Rule::next_state => {
                        let next_state = text.chars().next().unwrap();
                        if state_zero_flag == true {
                            state.on_zero.next_state = next_state;
                        } else {
                            state.on_one.next_state = next_state;
                            self.states.insert(state.id, state);
                            state = State::default();
                        }
                    },
                    _ => { unimplemented!("parse"); }
                }
            }
        }
    }

    fn step(&mut self) {
        let cursor_on_zero: bool;
        if !self.tape.contains(&self.cursor) {
            cursor_on_zero = true;
        } else {
            cursor_on_zero = false;
        }
        let rules: Rules;
        if let Some(state) = self.states.get(&self.current_state) {
            if cursor_on_zero == true {
                rules = state.on_zero.clone();
            } else {
                rules = state.on_one.clone();
            }
        } else {
            println!("Current state {} unknown, cannot determine rules", 
                     self.current_state);
            return;
        }
        if rules.write == 0 {
            self.tape.remove(&self.cursor);
        } else {
            self.tape.insert(self.cursor);
        }
        match rules.direction {
            Direction::Left => {
                self.cursor -= 1;
            },
            Direction::Right => {
                self.cursor += 1;
            }
        }
        self.current_state = rules.next_state;
    }

    fn run(&mut self) {
        while self.step_count < self.step_limit {
            self.step();
            self.step_count += 1;
        }
    }

    fn diagnostic_checksum(&self) -> usize {
        self.tape.len()
    }
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let mut machine = Machine::new();
    machine.parse(input.lines().collect());

    machine.run();
    println!("part 1: diagnostic checksum = {}", 
             machine.diagnostic_checksum());
}
