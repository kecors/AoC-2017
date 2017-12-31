use std::io::{stdin, Read};
use std::collections::{HashSet, HashMap};
use std::rc::Rc;
use std::cell::RefCell;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "program.pest"]
struct ProgramParser;

#[derive(Debug, Default)]
struct Program {
    name: String,
    weight: u32,
    disc: Vec<String>,
    disc_weight: Option<u32>
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
        disc: disc,
        disc_weight: None
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

// Perhaps don't need Rc<RefCell<>>? Just if let?
fn calculate_disc_weights(hm: &mut Rc<RefCell<HashMap<String, Program>>>, bottom: String) {
    let mut stack: Vec<String> = Vec::new();

    stack.push(bottom);

    loop {
        if stack.len() == 0 {
            break;
        }
        let name = stack.pop().unwrap();
//        println!("name = {}", name);

        let mut program = Program::default();
        if let Some(q) = hm.borrow().get(&name) {
//            println!("q = {:?}", q);
            program.name = q.name.clone();
            program.weight = q.weight;
            program.disc = q.disc.clone();
        }
//        println!("program = {:?}", program);

        if program.disc.len() == 0 {
            if let Some(p) = hm.borrow_mut().get_mut(&name) {
                p.disc_weight = Some(0);
                continue;
            }
        }

        let mut needs_calculation = Vec::new();
        for subname in program.disc.clone() {
            if let Some(subprogram) = hm.borrow().get(&subname) {
                if subprogram.disc_weight == None {
                    needs_calculation.push(subname);
                }
            }
        }
        if needs_calculation.len() > 0 {
            stack.push(name);
            for subname in needs_calculation {
                stack.push(subname);
            }
            continue;
        }

        let mut disc_weight = program.weight;
        for subname in program.disc.clone() {
            if let Some(subprogram) = hm.borrow().get(&subname) {
                disc_weight += subprogram.weight;
                if let Some(subweight) = subprogram.disc_weight {
                    disc_weight += subweight;
                }
            }
        }
        if let Some(p) = hm.borrow_mut().get_mut(&name) {
            p.disc_weight = Some(disc_weight);
        }
    }
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

    let mut k: Rc<RefCell<HashMap<String, Program>>> = Rc::new(RefCell::new(HashMap::new()));
    for program in programs {
        k.borrow_mut().insert(program.name.clone(), program);
    }
    println!("k = {:?}", k);
    calculate_disc_weights(&mut k, bottom);
    println!("");
    println!("k = {:?}", k);
}
