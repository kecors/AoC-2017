use std::io::{stdin, Read};

const DIVISOR: u64 = 2147483647;
const GEN_A_FACTOR: u64 = 16807;
const GEN_B_FACTOR: u64 = 48271;

#[derive(Debug)]
struct Generator {
    value: u64,
    factor: u64
}

impl Generator {
    fn new(start_value: u64, factor: u64) -> Generator {
        Generator {
            value: start_value,
            factor: factor
        }
    }

    fn create_next_value(&mut self) {
        self.value = (self.value * self.factor) % DIVISOR;
    }
}

fn execute(start_a: u64, start_b: u64) -> u32 {
    let mut matches: u32 = 0;
    let mut gen_a = Generator::new(start_a, GEN_A_FACTOR);
    let mut gen_b = Generator::new(start_b, GEN_B_FACTOR);

    for _ in 0..40000000 {
        gen_a.create_next_value();
        gen_b.create_next_value();
        if gen_a.value & 0xffff == gen_b.value & 0xffff {
            matches += 1;
        }
    }

    matches
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();
    let line_a = input.lines().next().unwrap();
    let start_a: u64 = line_a.split(' ').last().unwrap().parse().unwrap();
    let line_b = input.lines().last().unwrap();
    let start_b: u64 = line_b.split(' ').last().unwrap().parse().unwrap();

    let matches: u32 = execute(start_a, start_b);
    println!("part 1: matches = {}", matches);
}
