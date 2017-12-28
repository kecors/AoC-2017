use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct State {
    hm: HashMap<Vec<u32>, bool>
}

impl State {
    fn new() -> State {
        State {
            hm: HashMap::<Vec<u32>, bool>::new()
        }
    }

    fn track(&mut self, banks: &Vec<u32>) -> bool {
        match self.hm.entry((*banks).to_vec()) {
            Entry::Occupied(_) => return true,
            Entry::Vacant(v)   => v.insert(true)
        };
        false
    }
}

fn find_largest_bank(banks: &Vec<u32>) -> usize {
    let mut largest_bank: u32 = 0;
    let mut index_result: usize = 0;

    for (index, &bank) in banks.iter().enumerate() {
        if bank > largest_bank {
            largest_bank = bank;
            index_result = index;
        }
    }
    index_result
}

fn redistribute(banks: &mut Vec<u32>, source: usize) {
    let mut blocks: u32 = banks[source];
    banks[source] = 0;

    let mut index: usize = source;
    loop {
        index += 1;
        if index == banks.len() {
            index = 0;
        }
        banks[index] += 1;
        blocks -= 1;
        if blocks == 0 {
            return;
        }
    }
}

fn process(mut banks: Vec<u32>) {
    let mut state = State::new();
    let mut cycles: u32 = 0;

    loop {
        if state.track(&banks) == true {
            println!("Configuration repeated after {} cycles", cycles);
            return;
        }
        let index = find_largest_bank(&banks);
        redistribute(&mut banks, index);
        cycles += 1;
    }
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let strs: Vec<&str> = input.trim()
                               .split_whitespace()
                               .collect();
    let banks: Vec<u32> = strs.iter()
                              .map(|x| x.parse::<u32>().unwrap())
                              .collect();
    process(banks);
}
