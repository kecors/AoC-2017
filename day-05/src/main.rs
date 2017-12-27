use std::io::{stdin, Read};

fn jump_through_maze(instructions: &mut Vec<i32>) -> u32 {
    let mut instruction: i32;
    let mut index: usize = 0;
    let mut new_index: i32;
    let mut steps: u32 = 0;

    loop {
        instruction = instructions[index];
        steps += 1;

        new_index = (index as i32) + instruction;

        // Determine if we have exited the maze
        if new_index < 0 || new_index as usize >= instructions.len() {
            break;
        }

        instructions[index] += 1;

        index = new_index as usize;
    }

    steps
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let mut instructions: Vec<i32> = input.lines().map(|line| line.parse::<i32>().unwrap()).collect();

    let steps: u32 = jump_through_maze(&mut instructions);
    println!("steps = {}", steps);
}
