use std::io::{stdin, Read};

const DIVISOR: u64 = 2147483647;
const GEN_A_FACTOR: u64 = 16807;
const GEN_B_FACTOR: u64 = 48271;

#[derive(Debug)]
struct Generator {
    value: u64,
    factor: u64,
    criteria: u64
}

impl Generator {
    fn new(start_value: u64, factor: u64, criteria: u64) -> Generator {
        Generator {
            value: start_value,
            factor: factor,
            criteria: criteria
        }
    }

    fn create_next_value(&mut self) {
        loop {
            self.value = (self.value * self.factor) % DIVISOR;
            if self.value % self.criteria == 0 {
                break;
            }
        }
    }
}

fn execute(gen_a: &mut Generator, gen_b: &mut Generator, judge_limit: u64) -> u32 {
    let mut matches: u32 = 0;

    for _ in 0..judge_limit {
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

    let mut gen_a = Generator::new(start_a, GEN_A_FACTOR, 1);
    let mut gen_b = Generator::new(start_b, GEN_B_FACTOR, 1);
    let matches: u32 = execute(&mut gen_a, &mut gen_b, 40000000);
    println!("part 1: matches = {}", matches);

    let mut gen_a = Generator::new(start_a, GEN_A_FACTOR, 4);
    let mut gen_b = Generator::new(start_b, GEN_B_FACTOR, 8);
    let matches: u32 = execute(&mut gen_a, &mut gen_b, 5000000);
    println!("part 2: matches = {}", matches);
}
