use std::io::{stdin, Read};
use std::fmt::{Display, Error, Formatter};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Component {
    port_a: u32,
    port_b: u32,
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}/{}", self.port_a, self.port_b)
    }
}

impl Component {
    fn reversed(&self) -> Component {
        Component {
            port_a: self.port_b,
            port_b: self.port_a,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Interim {
    link_port: u32,
    bridge: Vec<Component>,
    used: HashSet<Component>,
}

impl Display for Interim {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut result: String = String::new();
        for component in self.bridge.iter() {
            result.push_str(&format!("{} -> ", component.port_a));
        }
        result.push_str(&format!("{}", self.link_port));
        write!(f, "{}", result)
    }
}

impl Interim {
    fn new() -> Interim {
        Interim::default()
    }

    fn sum(&self) -> u32 {
        let mut sum = 0;
        for component in self.bridge.iter() {
            sum += component.port_a;
            sum += component.port_b;
        }
        sum
    }
}

#[derive(Debug)]
struct State {
    components: Vec<Component>,
    port_hm: HashMap<u32, Vec<u32>>,
    maximum_weight: u32,
    longest_bridge_len: usize,
    longest_bridge_weight: u32,
}

impl State {
    fn new(components: Vec<Component>) -> State {
        let port_hm = make_port_hm(&components);
        State {
            components: components,
            port_hm: port_hm,
            maximum_weight: 0,
            longest_bridge_len: 0,
            longest_bridge_weight: 0,
        }
    }

    fn run(&mut self) {
        let mut stack: Vec<Interim> = Vec::new();
        stack.push(Interim::new());
        loop {
            if stack.len() == 0 {
                break;
            }
            let mut interim = Interim::new();
            if let Some(i) = stack.pop() {
                interim = i.clone();
            }
            let mut stack_add_flag = false;
            if let Some(tails) = self.port_hm.get(&interim.link_port) {
                for tail in tails {
                    let component = Component {
                        port_a: interim.link_port,
                        port_b: *tail,
                    };
                    if interim.used.contains(&component) {
                        continue;
                    }
                    let mut bridge = interim.bridge.clone();
                    bridge.push(component);
                    let mut used = interim.used.clone();
                    used.insert(component);
                    used.insert(component.reversed());
                    let interim = Interim {
                        bridge: bridge,
                        used: used,
                        link_port: *tail,
                    };
                    stack.push(interim);
                    stack_add_flag = true;
                }
            }
            if stack_add_flag == false {
                if interim.sum() > self.maximum_weight {
                    self.maximum_weight = interim.sum();
                }
                if interim.bridge.len() > self.longest_bridge_len {
                    self.longest_bridge_len = interim.bridge.len();
                    self.longest_bridge_weight = interim.sum();
                } else if interim.bridge.len() == self.longest_bridge_len {
                    if interim.sum() > self.longest_bridge_weight {
                        self.longest_bridge_weight = interim.sum();
                    }
                }
            }
        }
    }
}

fn make_port_hm(components: &Vec<Component>) -> HashMap<u32, Vec<u32>> {
    let mut hm = HashMap::new();
    for component in components {
        match hm.entry(component.port_a) {
            Entry::Vacant(vacant) => {
                let mut tail_ports = Vec::new();
                tail_ports.push(component.port_b);
                vacant.insert(tail_ports);
            }
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().push(component.port_b);
            }
        }
        match hm.entry(component.port_b) {
            Entry::Vacant(vacant) => {
                let mut tail_ports = Vec::new();
                tail_ports.push(component.port_a);
                vacant.insert(tail_ports);
            }
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().push(component.port_a);
            }
        }
    }
    hm
}

fn parse_line(line: &str) -> Component {
    let mut s = line.split('/');
    let port_a: u32 = s.next().unwrap().parse().unwrap();
    let port_b: u32 = s.next().unwrap().parse().unwrap();
    Component { port_a, port_b }
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let components: Vec<Component> = input.lines().map(|line| parse_line(line)).collect();

    let mut state = State::new(components);
    state.run();
    println!("part 1: maximum weight = {}", state.maximum_weight);
    println!(
        "part 2: longest bridge weight = {}",
        state.longest_bridge_weight
    );
}
