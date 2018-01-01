use std::io::{stdin, Read};
use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;

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

impl Program {
    fn total_weight(&self) -> u32 {
        self.weight + match self.disc_weight {
            Some(w) => w,
            None    => 0
        }
    }
}

fn parse_line(line: &str) -> Program {
    let mut program = Program::default();

    let pairs = ProgramParser::parse_str(Rule::line, line).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        let rule = pair.as_rule();
        let text = pair.clone().into_span().as_str().to_string();
        match rule {
            Rule::name       => { program.name = text; },
            Rule::weight     => { program.weight = text.parse().unwrap(); },
            Rule::subprogram => { program.disc.push(text); },
            _                => { println!("unknown rule {:?}", rule); }
        }
    }

    program
}

fn find_bottom(programs: &Vec<Program>) -> String {
    let mut programs_hs = HashSet::new();
    for program in programs {
        programs_hs.insert(program.name.clone());
    }

    let mut subprograms_hs = HashSet::new();
    for program in programs {
        for subprogram in program.disc.clone() {
            subprograms_hs.insert(subprogram);
        }
    }

    programs_hs.difference(&subprograms_hs).last().unwrap().clone()
}

fn calculate_disc_weights(hm: &mut HashMap<String, Program>, bottom: String) {
    let mut stack: Vec<String> = Vec::new();

    stack.push(bottom);

    loop {
        if stack.len() == 0 {
            break;
        }

        let name = stack.pop().unwrap();

        let mut program = Program::default();
        if let Some(p) = hm.get(&name) {
            program.name = p.name.clone();
            program.weight = p.weight;
            program.disc = p.disc.clone();
        }

        if program.disc.len() == 0 {
            if let Some(p) = hm.get_mut(&name) {
                p.disc_weight = Some(0);
                continue;
            }
        }

        let mut needs_calculation = Vec::new();
        for subname in program.disc.clone() {
            if let Some(subprogram) = hm.get(&subname) {
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

        let mut disc_weight = 0;
        for subname in program.disc.clone() {
            if let Some(subprogram) = hm.get(&subname) {
                disc_weight += subprogram.weight;
                if let Some(subweight) = subprogram.disc_weight {
                    disc_weight += subweight;
                }
            }
        }
        if let Some(p) = hm.get_mut(&name) {
            p.disc_weight = Some(disc_weight);
        }
    }
}

fn display_tower(hm: &HashMap<String, Program>, bottom: String) {
    let mut stack: Vec<(String, u32)> = Vec::new();

    stack.push((bottom, 0));

    loop {
        if stack.len() == 0 {
            break;
        }

        let (name, depth) = stack.pop().unwrap();

        let mut program = Program::default();
        if let Some(p) = hm.get(&name) {
            program.name = p.name.clone();
            program.weight = p.weight;
            program.disc = p.disc.clone();
            program.disc_weight = p.disc_weight;
        }

        for _ in 0..depth {
            program.name.insert(0, '-');
        }
        print!("{:12} ({:6}) [{:6}]", program.name, 
                                      program.weight, 
                                      program.disc_weight.unwrap());

        for subprogram in program.disc.clone() {
            print!(" {}", subprogram);
        }
        println!("");

        program.disc.reverse();
        for subprogram in program.disc {
            stack.push((subprogram, depth+1));
        }
    }
}

fn find_imbalance(hm: &HashMap<String, Program>, bottom: String) {
    let mut stack = Vec::new();
    let mut ancestry = Vec::new();
    let mut standards = Vec::new();

    stack.push(bottom);

    loop {
        if stack.len() == 0 {
            break;
        }

        let name = stack.pop().unwrap();

        ancestry.push(name.clone());

        let mut program = Program::default();
        if let Some(p) = hm.get(&name) {
            program.name = p.name.clone();
            program.weight = p.weight;
            program.disc = p.disc.clone();
            program.disc_weight = p.disc_weight;
        };

        let mut es: HashMap<u32, Vec<String>> = HashMap::new();
        for subprogram in program.disc {
            if let Some(p) = hm.get(&subprogram) {
                match es.entry(p.total_weight()) {
                    Entry::Vacant(v)   => { 
                        println!("v = {:?}", v);
                        let mut a = Vec::new();
                        a.push(p.name.clone());
                        v.insert(a); 
                    },
                    Entry::Occupied(mut o) => {
                        println!("o = {:?}", o);
                        o.get_mut().push(p.name.clone());
                    }
                };
            }
        }
        println!("es = {:?}", es);
        println!("es.len() = {}", es.len());
        if es.len() == 1 {
            println!("ancestry = {:?}", ancestry);
            println!("standards = {:?}", standards);
            for (k,v) in es {
                let result = standards.pop().unwrap() - (k * v.len() as u32);
                println!("result = {}", result);
            }
            break;
        }
        let mut standard: u32 = 0;
        let mut deviant: u32 = 0;
        for (k,v) in es {
            if v.len() == 1 {
                println!("v = {:?}", v);
                stack.push(v[0].clone());
                deviant = k;
            } else {
                standard = k;
            }
        }
        println!("standard = {}, deviant = {}", standard, deviant);
        standards.push(standard);
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
//    println!("bottom = {}", bottom);

    let mut hm: HashMap<String, Program> = HashMap::new();
    for program in programs {
        hm.insert(program.name.clone(), program);
    }
//    println!("hm = {:?}", hm);
    calculate_disc_weights(&mut hm, bottom.clone());
//    println!("");
//    println!("hm = {:?}", hm);
    display_tower(&hm, bottom.clone());
    find_imbalance(&hm, bottom.clone());
}
