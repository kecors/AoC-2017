use std::io;

#[derive(PartialEq)]
enum Direction {
    RIGHT,
    UP,
    LEFT,
    DOWN
}

fn create_new_square(square: (i32,i32), direction: &Direction) -> (i32,i32) {
    match direction {
        &Direction::RIGHT => {
            (square.0 + 1, square.1    )
        },
        &Direction::UP => {
            (square.0    , square.1 + 1)
        },
        &Direction::LEFT => {
            (square.0 - 1, square.1    )
        },
        &Direction::DOWN => {
            (square.0    , square.1 - 1)
        },
    }
}

fn build_grid(grid: &mut Vec<(i32,i32)>, limit: usize) {
    let mut square = (0,0);
    let mut direction = Direction::RIGHT;
    let mut distance = 1;
    let mut index: usize = 0;
    let mut step = 0;

    loop {
        let new_square = create_new_square(square, &direction);

        grid.push(square);

        square = new_square;

        step += 1;
        if step == distance {
            // Reset step, possibly increment distance, and change direction
            step = 0;
            if direction == Direction::UP || direction == Direction::DOWN {
                distance += 1;
            }
            direction = match direction {
                Direction::RIGHT => Direction::UP,
                Direction::UP    => Direction::LEFT,
                Direction::LEFT  => Direction::DOWN,
                Direction::DOWN  => Direction::RIGHT
            }
        }

        index += 1;
        if index == limit { break; }
    }
}

fn calculate_steps(square: (i32,i32)) -> u32 {
   (square.0.abs() + square.1.abs()) as u32
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let limit: usize = input.trim().parse::<usize>().unwrap();

    let mut grid: Vec<(i32,i32)> = Vec::with_capacity(limit as usize);

    build_grid(&mut grid, limit);

    println!("steps for {} = {}", limit, calculate_steps(grid[limit-1]));
}
