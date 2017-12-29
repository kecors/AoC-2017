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
    disc: Vec<String>   // May want to wrap in Option
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

fn collect_names(programs: &Vec<Program>) -> HashSet<String> {
    let mut hs = HashSet::new();

    for program in programs {
        hs.insert(program.name.clone());
    }

    hs
}

fn collect_heldprograms(programs: &Vec<Program>) -> HashSet<String> {
    let mut hs = HashSet::new();

    for program in programs {
        for heldprogram in program.disc.clone() {
            hs.insert(heldprogram);
        }
    }

    hs
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();
//    println!("input = {:#?}", input);

    let programs: Vec<Program> = input.lines()
                                      .map(|line| parse_line(line))
                                      .collect();
//    println!("programs = {:?}", programs);

    let names = collect_names(&programs);
//    println!("name = {:?}", names);

    let heldprograms = collect_heldprograms(&programs);
//    println!("heldprograms = {:?}", heldprograms);

    for bottom in names.difference(&heldprograms) {
        println!("bottom = {:?}", bottom);
    }
}
