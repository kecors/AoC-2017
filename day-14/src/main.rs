use std::io;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
mod knothash;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }
}

//
// This State struct and impl was adopted from code for day 12
//
#[derive(Debug, Default)]
struct State {
    pipes: HashMap<Position, Vec<Position>>,
}

impl State {
    fn new() -> State {
        State::default()
    }

    fn add_pipes(&mut self, id: Position, piped: Vec<Position>) {
        match self.pipes.entry(id) {
            Entry::Vacant(vacant) => {
                vacant.insert(piped);
            }
            Entry::Occupied(_) => {
                unimplemented!("add_pipes");
            }
        }
    }

    #[allow(dead_code)]
    fn count_connected(&self, id: Position) -> u32 {
        let mut connected = HashSet::new();
        let mut stack: Vec<Position> = Vec::new();

        stack.push(id);

        loop {
            match stack.pop() {
                None => {
                    break;
                }
                Some(id) => {
                    if let Some(piped) = self.pipes.get(&id) {
                        for pid in piped {
                            if connected.contains(pid) == false {
                                stack.push(pid.clone());
                            }
                        }
                    }
                    connected.insert(id);
                }
            }
        }

        connected.len() as u32
    }

    fn purge_group(&mut self, id: Position) {
        let mut stack: Vec<Position> = Vec::new();

        stack.push(id);

        loop {
            match stack.pop() {
                None => {
                    break;
                }
                Some(id) => {
                    if let Some(piped) = self.pipes.get(&id) {
                        stack.extend(piped.iter().cloned());
                    }
                    self.pipes.remove(&id);
                }
            }
        }
    }

    fn count_groups(&mut self) -> u32 {
        let mut group_count: u32 = 0;

        loop {
            let id: Position;
            if let Some(key) = self.pipes.keys().next() {
                id = key.clone();
            } else {
                break;
            }
            self.purge_group(id);
            group_count += 1;
        }

        group_count
    }
}

fn generate_hexstrings(key: &String) -> Vec<String> {
    let mut hexstrings: Vec<String> = Vec::new();
    for j in 0..128 {
        let input: String = format!("{}-{}", key.trim(), j);
        let result: String = knothash::make_hexstring(&input);
        hexstrings.push(result);
    }
    hexstrings
}

fn hexstring_to_binary(hexstring: &String) -> Vec<bool> {
    let mut result = Vec::new();

    for c in hexstring.chars() {
        match c {
            '0' => {
                result.extend(&[false, false, false, false]);
            }
            '1' => {
                result.extend(&[false, false, false, true]);
            }
            '2' => {
                result.extend(&[false, false, true, false]);
            }
            '3' => {
                result.extend(&[false, false, true, true]);
            }
            '4' => {
                result.extend(&[false, true, false, false]);
            }
            '5' => {
                result.extend(&[false, true, false, true]);
            }
            '6' => {
                result.extend(&[false, true, true, false]);
            }
            '7' => {
                result.extend(&[false, true, true, true]);
            }
            '8' => {
                result.extend(&[true, false, false, false]);
            }
            '9' => {
                result.extend(&[true, false, false, true]);
            }
            'a' => {
                result.extend(&[true, false, true, false]);
            }
            'b' => {
                result.extend(&[true, false, true, true]);
            }
            'c' => {
                result.extend(&[true, true, false, false]);
            }
            'd' => {
                result.extend(&[true, true, false, true]);
            }
            'e' => {
                result.extend(&[true, true, true, false]);
            }
            'f' => {
                result.extend(&[true, true, true, true]);
            }
            _ => {
                unimplemented!("hexstring_to_binary");
            }
        }
    }

    result
}

fn display_bits_vecs(bits_vecs: &Vec<Vec<bool>>) {
    for bits_vec in bits_vecs {
        print!("    ");
        for bit in bits_vec {
            match *bit {
                false => {
                    print!(".");
                }
                true => {
                    print!("#");
                }
            }
        }
        println!("");
    }
}

fn display_bits_total(bits_vecs: &Vec<Vec<bool>>) {
    let mut bits_total: u32 = 0;
    for bits_vec in bits_vecs {
        for bit in bits_vec {
            if *bit == true {
                bits_total += 1;
            }
        }
    }
    println!("part 1: bits_total = {}", bits_total);
}

fn main() {
    let mut key = String::new();
    io::stdin().read_line(&mut key).unwrap();

    let hexstrings: Vec<String> = generate_hexstrings(&key);

    let mut bits_vecs: Vec<Vec<bool>> = Vec::new();
    for hexstring in hexstrings {
        bits_vecs.push(hexstring_to_binary(&hexstring));
    }

    display_bits_total(&bits_vecs);

    display_bits_vecs(&bits_vecs);

    let mut state = State::new();

    for x in 0..128 {
        for y in 0..128 {
            if bits_vecs[x][y] == true {
                let mut piped = Vec::new();
                if y < 127 && bits_vecs[x][y + 1] == true {
                    piped.push(Position::new(x as u32, (y + 1) as u32));
                }
                if y > 0 && bits_vecs[x][y - 1] == true {
                    piped.push(Position::new(x as u32, (y - 1) as u32));
                }
                if x < 127 && bits_vecs[x + 1][y] == true {
                    piped.push(Position::new((x + 1) as u32, y as u32));
                }
                if x > 0 && bits_vecs[x - 1][y] == true {
                    piped.push(Position::new((x - 1) as u32, y as u32));
                }
                state.add_pipes(Position::new(x as u32, y as u32), piped);
            }
        }
    }

    let result = state.count_groups();
    println!("part 2: group count = {}", result);
}
