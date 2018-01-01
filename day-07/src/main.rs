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

struct Tower {
    bottom: String,
    programs_hm: HashMap<String, Program>
}

impl Tower {
    fn new(programs: Vec<Program>) -> Tower {
        let bottom: String = Tower::find_bottom(&programs);

        let mut programs_hm: HashMap<String, Program> = HashMap::new();
        for program in programs {
            programs_hm.insert(program.name.clone(), program);
        }

        Tower {
            bottom: bottom,
            programs_hm: programs_hm
        }
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

    fn calculate_disc_weights(&mut self) {
        let mut stack: Vec<String> = Vec::new();

        stack.push(self.bottom.clone());

        loop {
            if stack.len() == 0 {
                break;
            }

            let name = stack.pop().unwrap();

            let mut program = Program::default();
            if let Some(p) = self.programs_hm.get(&name) {
                program.name = p.name.clone();
                program.weight = p.weight;
                program.disc = p.disc.clone();
            }

            if program.disc.len() == 0 {
                if let Some(p) = self.programs_hm.get_mut(&name) {
                    p.disc_weight = Some(0);
                    continue;
                }
            }

            let mut needs_calculation = Vec::new();
            for subname in program.disc.clone() {
                if let Some(subprogram) = self.programs_hm.get(&subname) {
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
                if let Some(subprogram) = self.programs_hm.get(&subname) {
                    disc_weight += subprogram.weight;
                    if let Some(subweight) = subprogram.disc_weight {
                        disc_weight += subweight;
                    }
                }
            }
            if let Some(p) = self.programs_hm.get_mut(&name) {
                p.disc_weight = Some(disc_weight);
            }
        }
    }

    fn display(&self) {
        let mut stack: Vec<(String, u32)> = Vec::new();

        stack.push((self.bottom.clone(), 0));

        loop {
            if stack.len() == 0 {
                break;
            }

            let (name, depth) = stack.pop().unwrap();

            let mut program = Program::default();
            if let Some(p) = self.programs_hm.get(&name) {
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

    fn find_imbalance(&self) {
        let mut stack = Vec::new();
        let mut standards = Vec::new();

        stack.push(self.bottom.clone());

        loop {
            if stack.len() == 0 {
                break;
            }

            let name = stack.pop().unwrap();

            let mut program = Program::default();
            if let Some(p) = self.programs_hm.get(&name) {
                program.name = p.name.clone();
                program.weight = p.weight;
                program.disc = p.disc.clone();
                program.disc_weight = p.disc_weight;
            };

            let mut weights_hm: HashMap<u32, Vec<String>> = HashMap::new();
            for subprogram in program.disc {
                if let Some(p) = self.programs_hm.get(&subprogram) {
                    match weights_hm.entry(p.total_weight()) {
                        Entry::Vacant(vacant)   => { 
                            let mut subprograms = Vec::new();
                            subprograms.push(p.name.clone());
                            vacant.insert(subprograms); 
                        },
                        Entry::Occupied(mut occupied) => {
                            occupied.get_mut().push(p.name.clone());
                        }
                    };
                }
            }
            println!("{} -> {:?}", name, weights_hm);
            if weights_hm.len() == 1 {
                for (k,v) in weights_hm {
                    let standard = standards.pop().unwrap();
                    let result = standard - (k * v.len() as u32);
                    println!("result: {} - ({} * {}) = {}",
                        standard, k, v.len(), result);
                }
                break;
            }
            for (k,v) in weights_hm {
                if v.len() == 1 {
                    stack.push(v[0].clone());
                } else {
                    standards.push(k);
                }
            }
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

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let programs: Vec<Program> = input.lines()
                                      .map(|line| parse_line(line))
                                      .collect();

    let mut tower = Tower::new(programs);
    tower.calculate_disc_weights();
    tower.display();
    tower.find_imbalance();
}
