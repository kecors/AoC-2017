use std::io;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "stream.pest"]
struct StreamParser;

#[derive(Debug, PartialEq)]
enum Token {
    GroupStart,
    GroupEnd,
    GarbageStart,
    GarbageEnd,
    Cancel,
    Other,
}

#[derive(Debug, Default)]
struct State {
    cancel: bool,
    garbage: bool,
    depth: u32,
    scores: Vec<u32>,
    garbage_total: u32,
}

impl State {
    fn new() -> State {
        State {
            cancel: false,
            garbage: false,
            depth: 0,
            scores: Vec::new(),
            garbage_total: 0,
        }
    }

    fn process(&mut self, token: Token) {
        //println!("state = {:?}, token = {:?}", self, token);
        if self.cancel == true {
            self.cancel = false;
            return;
        }

        if token == Token::Cancel {
            self.cancel = true;
            return;
        }

        if self.garbage == true {
            if token == Token::GarbageEnd {
                self.garbage = false;
            } else {
                self.garbage_total += 1;
            }
            return;
        }

        match token {
            Token::GroupStart => {
                self.depth += 1;
            }
            Token::GroupEnd => {
                self.scores.push(self.depth);
                self.depth -= 1;
            }
            Token::GarbageStart => {
                self.garbage = true;
            }
            _ => {}
        }
    }

    fn total_score(&self) -> u32 {
        self.scores.iter().sum()
    }

    fn total_garbage(&self) -> u32 {
        self.garbage_total
    }
}

fn parse_line(stream: &str, state: &mut State) {
    let pairs = StreamParser::parse_str(Rule::stream, stream).unwrap_or_else(|e| panic!("{}", e));

    //println!("pairs = {:?}", pairs);
    for pair in pairs {
        state.process(match pair.as_rule() {
            Rule::groupstart => Token::GroupStart,
            Rule::groupend => Token::GroupEnd,
            Rule::garbagestart => Token::GarbageStart,
            Rule::garbageend => Token::GarbageEnd,
            Rule::cancel => Token::Cancel,
            _ => Token::Other,
        });
    }
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    //println!("input = {:?}", input.trim());

    let mut state = State::new();
    parse_line(input.trim(), &mut state);
    //println!("state = {:?}", state);
    println!("total score = {}", state.total_score());
    println!("total garbage = {}", state.total_garbage());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let mut state = State::new();
        parse_line("{}", &mut state);
        assert_eq!(1, state.total_score());
    }

    #[test]
    fn part1_example_2() {
        let mut state = State::new();
        parse_line("{{{}}}", &mut state);
        assert_eq!(6, state.total_score());
    }

    #[test]
    fn part1_example_3() {
        let mut state = State::new();
        parse_line("{{},{}}", &mut state);
        assert_eq!(5, state.total_score());
    }

    #[test]
    fn part1_example_4() {
        let mut state = State::new();
        parse_line("{{{},{},{{}}}}", &mut state);
        assert_eq!(16, state.total_score());
    }

    #[test]
    fn part1_example_5() {
        let mut state = State::new();
        parse_line("{<a>,<a>,<a>,<a>}", &mut state);
        assert_eq!(1, state.total_score());
    }

    #[test]
    fn part1_example_6() {
        let mut state = State::new();
        parse_line("{{<ab>},{<ab>},{<ab>},{<ab>}}", &mut state);
        assert_eq!(9, state.total_score());
    }

    #[test]
    fn part1_example_7() {
        let mut state = State::new();
        parse_line("{{<!!>},{<!!>},{<!!>},{<!!>}}", &mut state);
        assert_eq!(9, state.total_score());
    }

    #[test]
    fn part1_example_8() {
        let mut state = State::new();
        parse_line("{{<a!>},{<a!>},{<a!>},{<ab>}}", &mut state);
        assert_eq!(3, state.total_score());
    }

    #[test]
    fn part2_example_1() {
        let mut state = State::new();
        parse_line("<>", &mut state);
        assert_eq!(0, state.total_garbage());
    }

    #[test]
    fn part2_example_2() {
        let mut state = State::new();
        parse_line("<random characters>", &mut state);
        assert_eq!(17, state.total_garbage());
    }

    #[test]
    fn part2_example_3() {
        let mut state = State::new();
        parse_line("<<<<>", &mut state);
        assert_eq!(3, state.total_garbage());
    }

    #[test]
    fn part2_example_4() {
        let mut state = State::new();
        parse_line("<{!>}>", &mut state);
        assert_eq!(2, state.total_garbage());
    }

    #[test]
    fn part2_example_5() {
        let mut state = State::new();
        parse_line("<!!>", &mut state);
        assert_eq!(0, state.total_garbage());
    }

    #[test]
    fn part2_example_6() {
        let mut state = State::new();
        parse_line("<!!!>>", &mut state);
        assert_eq!(0, state.total_garbage());
    }

    #[test]
    fn part2_example_7() {
        let mut state = State::new();
        parse_line("<{o\"i!a,<}i<a>", &mut state);
        assert_eq!(10, state.total_garbage());
    }
}
