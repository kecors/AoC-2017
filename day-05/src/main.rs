//
// To run part 1:
//     cat puzzle-input.txt | cargo run
//
// To run part 2:
//     cat puzzle-input.txt | cargo run --features part2
//

use std::io::{stdin, Read};

#[cfg(feature = "part2")]
fn modify_instruction(instructions: &mut Vec<i32>, index: usize, instruction: i32) {
    if instruction >= 3 {
        instructions[index] -= 1;
    } else {
        instructions[index] += 1;
    }
}

#[cfg(not(feature = "part2"))]
fn modify_instruction(instructions: &mut Vec<i32>, index: usize, _instruction: i32) {
    instructions[index] += 1;
}

fn jump_through_maze(mut instructions: &mut Vec<i32>) -> u32 {
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

        modify_instruction(&mut instructions, index, instruction);

        index = new_index as usize;
    }

    steps
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let mut instructions: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let steps: u32 = jump_through_maze(&mut instructions);
    println!("steps = {}", steps);
}
