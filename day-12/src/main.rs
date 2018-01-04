use std::io::{stdin, Read};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "linking.pest"]
struct LinkingParser;

#[derive(Debug, Default)]
struct Linking {
    id: u32,
    piped: Vec<u32>
}

#[derive(Debug, Default)]
struct State {
    pipes: HashMap<u32, Vec<u32>>
}

impl State {
    fn new() -> State {
        State::default()
    }

    fn add_pipes(&mut self, id: u32, piped: Vec<u32>) {
        match self.pipes.entry(id) {
            Entry::Vacant(vacant) => { vacant.insert(piped); },
            Entry::Occupied(_)    => { println!("occupied unexpectedly"); }
        }
    }

    fn count_connected(&self, id: u32) -> u32 {
        let mut connected = HashSet::new();
        let mut stack: Vec<u32> = Vec::new();

        stack.push(id);

        loop {
            match stack.pop() {
                None     => { break; },
                Some(id) => {
                    if let Some(piped) = self.pipes.get(&id) {
                        for pid in piped {
                            if connected.contains(pid) == false {
                                stack.push(*pid);
                            }
                        }
                    }
                    connected.insert(id);
                }
            }
        }

        connected.len() as u32
    }

    fn purge_group(&mut self, id: u32) {
        let mut stack: Vec<u32> = Vec::new();

        stack.push(id);

        loop {
            match stack.pop() {
                None     => { break; },
                Some(id) => {
                    if let Some(piped) = self.pipes.get(&id) {
                        stack.extend(piped);
                    }
                    self.pipes.remove(&id);
                }
            }
        }
    }

    fn count_groups(&mut self) -> u32 {
        let mut group_count: u32 = 0;

        loop {
            let id: u32;
            if let Some(key) = self.pipes.keys().next() {
                id = *key;
            } else {
                break;
            }
            self.purge_group(id);
            group_count += 1;
        }

        group_count
    }
}

fn parse_line(line: &str) -> Linking {
    let pairs = LinkingParser::parse_str(Rule::linking, line).unwrap_or_else(|e| panic!("{}", e));

    let mut linking = Linking::default();

    for pair in pairs {
        let rule = pair.as_rule();
        let text = pair.clone().into_span().as_str().to_string();
        match rule {
            Rule::id    => { linking.id = text.parse().unwrap(); },
            Rule::piped => { linking.piped.push(text.parse().unwrap()); },
            _           => { println!("unknown rule {:?}", rule); }
        }
    }

    linking
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let linkings: Vec<Linking> = input.lines()
                                      .map(|line| parse_line(line))
                                      .collect();
//    println!("linkings = {:?}", linkings);

    let mut state = State::new();
    for linking in linkings {
        state.add_pipes(linking.id, linking.piped);
    }
//    println!("state = {:?}", state);
    println!("programs connected to {} = {}", 0, state.count_connected(0));

    println!("group count = {}", state.count_groups());
}
