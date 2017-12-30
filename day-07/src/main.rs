use std::io::{stdin, Read};
use std::collections::HashSet;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "program.pest"]
struct ProgramParser;

#[derive(Debug)]
struct Program {
    name: String,
    weight: u32,
    disc: Vec<String>
}

fn parse_line(line: &str) -> Program {
//    println!("line = {:?}", line);

    let mut name: String = "unspecified".to_string();
    let mut weight: u32 = 0;
    let mut disc: Vec<String> = Vec::new();

    let pairs = ProgramParser::parse_str(Rule::line, line).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        let rule = pair.as_rule();
        let text = pair.clone().into_span().as_str().to_string();
        match rule {
            Rule::name        => { name = text; },
            Rule::weight      => { weight = text.parse::<u32>().unwrap(); },
            Rule::heldprogram => { disc.push(text); },
            _                 => { println!("unknown rule {:?}", rule); }
        }
    }
    Program {
        name: name,
        weight: weight,
        disc: disc
    }
}

fn find_bottom(programs: &Vec<Program>) -> String {
    let mut programs_hs = HashSet::new();
    for program in programs {
        programs_hs.insert(program.name.clone());
    }

    let mut heldprograms_hs = HashSet::new();
    for program in programs {
        for heldprogram in program.disc.clone() {
            heldprograms_hs.insert(heldprogram);
        }
    }

    programs_hs.difference(&heldprograms_hs).last().unwrap().clone()
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();
//    println!("input = {:#?}", input);

    let programs: Vec<Program> = input.lines()
                                      .map(|line| parse_line(line))
                                      .collect();
//    println!("programs = {:?}", programs);

    let bottom = find_bottom(&programs);
    println!("bottom = {}", bottom);
}
